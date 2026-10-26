//! Pure rules of the level of detail (LOD) of the 3D scene: which representation a block
//! or an item gets at a given distance from the camera, and which labels are worth drawing.
//!
//! The Bevy side (`lod.rs` of the `utsim-view` binary) applies them to the entities. The
//! thresholds are in metres and live in [`LodSettings`], so they can be tuned (and
//! switched off) without touching the systems.

/// How a block is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockDetail {
    /// The realistic model (a glTF scene).
    Model,
    /// A plain box with the block's footprint and height.
    Box,
}

/// How an item is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemDetail {
    /// The realistic item (a pallet with its boxes, or one box).
    Model,
    /// A plain cube.
    Box,
    /// Not drawn.
    Hidden,
}

/// Distances (metres) and counts that decide the LOD.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LodSettings {
    /// The whole LOD is on. Off draws everything in full at any distance.
    pub enabled: bool,
    /// The LOD only acts on a model this heavy: at least this many realistic blocks
    /// or [`LodSettings::min_items`] items drawn. A plant of a dozen blocks is seen in
    /// full from any distance; `0` makes the LOD act always.
    pub min_blocks: usize,
    pub min_items: usize,
    /// Beyond this distance a realistic block becomes a box.
    pub block_box_m: f32,
    /// Beyond this distance a realistic item becomes a cube.
    pub item_box_m: f32,
    /// Beyond this distance items are not drawn.
    pub item_hide_m: f32,
    /// Beyond this distance labels are not drawn.
    pub label_hide_m: f32,
    /// At most this many labels are drawn (the nearest ones).
    pub label_max: usize,
}

impl Default for LodSettings {
    fn default() -> Self {
        LodSettings {
            enabled: true,
            min_blocks: 100,
            min_items: 400,
            block_box_m: 45.0,
            item_box_m: 30.0,
            item_hide_m: 140.0,
            label_hide_m: 90.0,
            label_max: 150,
        }
    }
}

/// A level changes back only after the camera has come this much closer than the
/// threshold that sent it away, so an entity at the border does not flicker.
const HYSTERESIS: f32 = 0.9;

impl LodSettings {
    /// These settings with `enabled` set to whether the LOD acts on a scene of `blocks`
    /// realistic blocks and `items` drawn items.
    pub fn engaged(&self, blocks: usize, items: usize) -> LodSettings {
        LodSettings {
            enabled: self.enabled && (blocks >= self.min_blocks || items >= self.min_items),
            ..*self
        }
    }

    /// Detail of a block at `dist` metres, given the `current` one (for the hysteresis).
    pub fn block_detail(&self, dist: f32, current: BlockDetail) -> BlockDetail {
        if !self.enabled {
            return BlockDetail::Model;
        }
        let limit = match current {
            BlockDetail::Model => self.block_box_m,
            BlockDetail::Box => self.block_box_m * HYSTERESIS,
        };
        if dist > limit {
            BlockDetail::Box
        } else {
            BlockDetail::Model
        }
    }

    /// Detail of an item at `dist` metres, given the `current` one.
    pub fn item_detail(&self, dist: f32, current: ItemDetail) -> ItemDetail {
        if !self.enabled {
            return ItemDetail::Model;
        }
        // Going to a farther level needs the threshold; coming back needs 10 % less.
        let hide = if current == ItemDetail::Hidden {
            self.item_hide_m * HYSTERESIS
        } else {
            self.item_hide_m
        };
        if dist > hide {
            return ItemDetail::Hidden;
        }
        let to_box = if current == ItemDetail::Model {
            self.item_box_m
        } else {
            self.item_box_m * HYSTERESIS
        };
        if dist > to_box {
            ItemDetail::Box
        } else {
            ItemDetail::Model
        }
    }

    /// Which of `distances` (one per label) are drawn: the nearest `label_max` of those
    /// within `label_hide_m`. With the LOD off, all of them.
    pub fn label_mask(&self, distances: &[f32]) -> Vec<bool> {
        if !self.enabled {
            return vec![true; distances.len()];
        }
        let mut mask: Vec<bool> = distances.iter().map(|d| *d <= self.label_hide_m).collect();
        let near = mask.iter().filter(|m| **m).count();
        if near > self.label_max {
            let mut within: Vec<f32> = distances
                .iter()
                .zip(&mask)
                .filter(|(_, m)| **m)
                .map(|(d, _)| *d)
                .collect();
            // The distance of the label_max-th nearest is the cut; ties are cut by index.
            let (_, cut, _) = within.select_nth_unstable_by(self.label_max - 1, f32::total_cmp);
            let cut = *cut;
            let mut kept = 0;
            for (m, d) in mask.iter_mut().zip(distances) {
                if *m {
                    if *d <= cut && kept < self.label_max {
                        kept += 1;
                    } else {
                        *m = false;
                    }
                }
            }
        }
        mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_become_boxes_far_away_with_hysteresis() {
        let s = LodSettings::default();
        assert_eq!(s.block_detail(10.0, BlockDetail::Model), BlockDetail::Model);
        assert_eq!(s.block_detail(60.0, BlockDetail::Model), BlockDetail::Box);
        // Between 0.9 x and 1.0 x of the threshold the level does not change.
        let mid = s.block_box_m * 0.95;
        assert_eq!(s.block_detail(mid, BlockDetail::Model), BlockDetail::Model);
        assert_eq!(s.block_detail(mid, BlockDetail::Box), BlockDetail::Box);
        assert_eq!(s.block_detail(10.0, BlockDetail::Box), BlockDetail::Model);
    }

    #[test]
    fn items_go_model_box_hidden() {
        let s = LodSettings::default();
        assert_eq!(s.item_detail(5.0, ItemDetail::Model), ItemDetail::Model);
        assert_eq!(s.item_detail(50.0, ItemDetail::Model), ItemDetail::Box);
        assert_eq!(s.item_detail(200.0, ItemDetail::Model), ItemDetail::Hidden);
        assert_eq!(s.item_detail(200.0, ItemDetail::Box), ItemDetail::Hidden);
        assert_eq!(s.item_detail(10.0, ItemDetail::Hidden), ItemDetail::Model);
        assert_eq!(s.item_detail(50.0, ItemDetail::Hidden), ItemDetail::Box);
        let mid = s.item_box_m * 0.95;
        assert_eq!(s.item_detail(mid, ItemDetail::Box), ItemDetail::Box);
        assert_eq!(s.item_detail(mid, ItemDetail::Model), ItemDetail::Model);
    }

    #[test]
    fn small_models_are_left_alone() {
        let s = LodSettings::default();
        assert!(!s.engaged(16, 30).enabled);
        assert!(s.engaged(100, 0).enabled);
        assert!(s.engaged(10, 400).enabled);
        let never = LodSettings {
            enabled: false,
            ..s
        };
        assert!(!never.engaged(1000, 1000).enabled);
        let always = LodSettings { min_blocks: 0, ..s };
        assert!(always.engaged(1, 0).enabled);
        // A small model keeps every label and every model.
        let small = s.engaged(16, 30);
        assert_eq!(
            small.block_detail(1e5, BlockDetail::Model),
            BlockDetail::Model
        );
        assert_eq!(small.label_mask(&[1e5]), vec![true]);
    }

    #[test]
    fn off_draws_everything() {
        let s = LodSettings {
            enabled: false,
            ..LodSettings::default()
        };
        assert_eq!(s.block_detail(1e6, BlockDetail::Box), BlockDetail::Model);
        assert_eq!(s.item_detail(1e6, ItemDetail::Hidden), ItemDetail::Model);
        assert_eq!(s.label_mask(&[1.0, 1e6]), vec![true, true]);
    }

    #[test]
    fn labels_keep_the_nearest() {
        let s = LodSettings {
            label_max: 3,
            label_hide_m: 100.0,
            ..LodSettings::default()
        };
        let d = [50.0, 5.0, 200.0, 20.0, 10.0, 30.0];
        // 200 m is too far; of the other five the nearest three are 5, 10, 20.
        assert_eq!(
            s.label_mask(&d),
            vec![false, true, false, true, true, false]
        );
        // Fewer than the cap: only the distance cut applies.
        let few = [1.0, 300.0];
        assert_eq!(s.label_mask(&few), vec![true, false]);
    }

    #[test]
    fn equal_distances_never_exceed_the_cap() {
        let s = LodSettings {
            label_max: 2,
            ..LodSettings::default()
        };
        let mask = s.label_mask(&[10.0, 10.0, 10.0, 10.0]);
        assert_eq!(mask.iter().filter(|m| **m).count(), 2);
    }
}
