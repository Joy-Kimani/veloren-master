mod block_mask;
mod castle;
pub mod economy;
pub mod namegen;
pub mod settlement;
mod tree;

// Reexports
pub use self::{
    block_mask::BlockMask, castle::Castle, economy::Economy, settlement::Settlement, tree::Tree,
};

pub use common::terrain::site::{DungeonKindMeta, SettlementKindMeta, SiteKindMeta};

use crate::{Canvas, column::ColumnSample, site2};
use common::{calendar::Calendar, generation::ChunkSupplement, resources::TimeOfDay};
use rand::Rng;
use serde::Deserialize;
use vek::*;

#[derive(Deserialize)]
pub struct Colors {
    pub castle: castle::Colors,
    pub settlement: settlement::Colors,
}

pub struct SpawnRules {
    pub trees: bool,
    pub max_warp: f32,
    pub paths: bool,
    pub waypoints: bool,
}

impl SpawnRules {
    #[must_use]
    pub fn combine(self, other: Self) -> Self {
        // Should be commutative
        Self {
            trees: self.trees && other.trees,
            max_warp: self.max_warp.min(other.max_warp),
            paths: self.paths && other.paths,
            waypoints: self.waypoints && other.waypoints,
        }
    }
}

impl Default for SpawnRules {
    fn default() -> Self {
        Self {
            trees: true,
            max_warp: 1.0,
            paths: true,
            waypoints: true,
        }
    }
}

pub struct Site {
    pub kind: SiteKind,
    pub economy: Economy,
}

pub enum SiteKind {
    Settlement(Settlement),
    Castle(Castle),
    Refactor(site2::Site),
    CliffTown(site2::Site),
    SavannahTown(site2::Site),
    Tree(Tree),
    DesertCity(site2::Site),
    ChapelSite(site2::Site),
    DwarvenMine(site2::Site),
    CoastalTown(site2::Site),
    Terracotta(site2::Site),
    GiantTree(site2::Site),
    Gnarling(site2::Site),
    Bridge(site2::Site),
    Adlet(site2::Site),
    Haniwa(site2::Site),
    PirateHideout(site2::Site),
    JungleRuin(site2::Site),
    RockCircle(site2::Site),
    TrollCave(site2::Site),
    Camp(site2::Site),
    Cultist(site2::Site),
    Sahagin(site2::Site),
    VampireCastle(site2::Site),
    GliderCourse(site2::Site),
    Myrmidon(site2::Site),
}

impl Site {
    pub fn settlement(s: Settlement) -> Self {
        Self {
            kind: SiteKind::Settlement(s),
            economy: Economy::default(),
        }
    }

    pub fn gnarling(g: site2::Site) -> Self {
        Self {
            kind: SiteKind::Gnarling(g),
            economy: Economy::default(),
        }
    }

    pub fn adlet(ad: site2::Site) -> Self {
        Self {
            kind: SiteKind::Adlet(ad),
            economy: Economy::default(),
        }
    }

    pub fn haniwa(ha: site2::Site) -> Self {
        Self {
            kind: SiteKind::Haniwa(ha),
            economy: Economy::default(),
        }
    }

    pub fn castle(c: Castle) -> Self {
        Self {
            kind: SiteKind::Castle(c),
            economy: Economy::default(),
        }
    }

    pub fn refactor(s: site2::Site) -> Self {
        Self {
            kind: SiteKind::Refactor(s),
            economy: Economy::default(),
        }
    }

    pub fn cliff_town(ct: site2::Site) -> Self {
        Self {
            kind: SiteKind::CliffTown(ct),
            economy: Economy::default(),
        }
    }

    pub fn savannah_town(st: site2::Site) -> Self {
        Self {
            kind: SiteKind::SavannahTown(st),
            economy: Economy::default(),
        }
    }

    pub fn coastal_town(ct: site2::Site) -> Self {
        Self {
            kind: SiteKind::CoastalTown(ct),
            economy: Economy::default(),
        }
    }

    pub fn pirate_hideout(ph: site2::Site) -> Self {
        Self {
            kind: SiteKind::PirateHideout(ph),
            economy: Economy::default(),
        }
    }

    pub fn jungle_ruin(jr: site2::Site) -> Self {
        Self {
            kind: SiteKind::JungleRuin(jr),
            economy: Economy::default(),
        }
    }

    pub fn rock_circle(rc: site2::Site) -> Self {
        Self {
            kind: SiteKind::RockCircle(rc),
            economy: Economy::default(),
        }
    }

    pub fn troll_cave(tc: site2::Site) -> Self {
        Self {
            kind: SiteKind::TrollCave(tc),
            economy: Economy::default(),
        }
    }

    pub fn camp(cp: site2::Site) -> Self {
        Self {
            kind: SiteKind::Camp(cp),
            economy: Economy::default(),
        }
    }

    pub fn desert_city(dc: site2::Site) -> Self {
        Self {
            kind: SiteKind::DesertCity(dc),
            economy: Economy::default(),
        }
    }

    pub fn chapel_site(p: site2::Site) -> Self {
        Self {
            kind: SiteKind::ChapelSite(p),
            economy: Economy::default(),
        }
    }

    pub fn cultist(cl: site2::Site) -> Self {
        Self {
            kind: SiteKind::Cultist(cl),
            economy: Economy::default(),
        }
    }

    pub fn sahagin(sg: site2::Site) -> Self {
        Self {
            kind: SiteKind::Sahagin(sg),
            economy: Economy::default(),
        }
    }

    pub fn vampire_castle(vc: site2::Site) -> Self {
        Self {
            kind: SiteKind::VampireCastle(vc),
            economy: Economy::default(),
        }
    }

    pub fn myrmidon(my: site2::Site) -> Self {
        Self {
            kind: SiteKind::Myrmidon(my),
            economy: Economy::default(),
        }
    }

    pub fn dwarven_mine(dm: site2::Site) -> Self {
        Self {
            kind: SiteKind::DwarvenMine(dm),
            economy: Economy::default(),
        }
    }

    pub fn terracotta(tr: site2::Site) -> Self {
        Self {
            kind: SiteKind::Terracotta(tr),
            economy: Economy::default(),
        }
    }

    pub fn tree(t: Tree) -> Self {
        Self {
            kind: SiteKind::Tree(t),
            economy: Economy::default(),
        }
    }

    pub fn giant_tree(gt: site2::Site) -> Self {
        Self {
            kind: SiteKind::GiantTree(gt),
            economy: Economy::default(),
        }
    }

    pub fn bridge(b: site2::Site) -> Self {
        Self {
            kind: SiteKind::Bridge(b),
            economy: Economy::default(),
        }
    }

    pub fn glider_course(g: site2::Site) -> Self {
        Self {
            kind: SiteKind::GliderCourse(g),
            economy: Economy::default(),
        }
    }

    pub fn radius(&self) -> f32 {
        match &self.kind {
            SiteKind::Settlement(s) => s.radius(),
            SiteKind::Castle(c) => c.radius(),
            SiteKind::Refactor(s) => s.radius(),
            SiteKind::CliffTown(ct) => ct.radius(),
            SiteKind::SavannahTown(st) => st.radius(),
            SiteKind::CoastalTown(ct) => ct.radius(),
            SiteKind::PirateHideout(ph) => ph.radius(),
            SiteKind::JungleRuin(jr) => jr.radius(),
            SiteKind::RockCircle(rc) => rc.radius(),
            SiteKind::TrollCave(tc) => tc.radius(),
            SiteKind::Camp(cp) => cp.radius(),
            SiteKind::DesertCity(dc) => dc.radius(),
            SiteKind::ChapelSite(p) => p.radius(),
            SiteKind::DwarvenMine(dm) => dm.radius(),
            SiteKind::Terracotta(tr) => tr.radius(),
            SiteKind::Tree(t) => t.radius(),
            SiteKind::GiantTree(gt) => gt.radius(),
            SiteKind::Gnarling(g) => g.radius(),
            SiteKind::Bridge(b) => b.radius(),
            SiteKind::Adlet(ad) => ad.radius(),
            SiteKind::Haniwa(ha) => ha.radius(),
            SiteKind::Cultist(cl) => cl.radius(),
            SiteKind::Sahagin(sg) => sg.radius(),
            SiteKind::VampireCastle(vc) => vc.radius(),
            SiteKind::GliderCourse(gc) => gc.radius(),
            SiteKind::Myrmidon(my) => my.radius(),
        }
    }

    pub fn get_origin(&self) -> Vec2<i32> {
        match &self.kind {
            SiteKind::Settlement(s) => s.get_origin(),
            SiteKind::Castle(c) => c.get_origin(),
            SiteKind::Refactor(s) => s.origin,
            SiteKind::CliffTown(ct) => ct.origin,
            SiteKind::SavannahTown(st) => st.origin,
            SiteKind::CoastalTown(ct) => ct.origin,
            SiteKind::PirateHideout(ph) => ph.origin,
            SiteKind::JungleRuin(jr) => jr.origin,
            SiteKind::RockCircle(rc) => rc.origin,
            SiteKind::TrollCave(tc) => tc.origin,
            SiteKind::Camp(cp) => cp.origin,
            SiteKind::DesertCity(dc) => dc.origin,
            SiteKind::ChapelSite(p) => p.origin,
            SiteKind::DwarvenMine(dm) => dm.origin,
            SiteKind::Terracotta(tr) => tr.origin,
            SiteKind::Tree(t) => t.origin,
            SiteKind::GiantTree(gt) => gt.origin,
            SiteKind::Gnarling(g) => g.origin,
            SiteKind::Bridge(b) => b.origin,
            SiteKind::Adlet(ad) => ad.origin,
            SiteKind::Haniwa(ha) => ha.origin,
            SiteKind::Cultist(cl) => cl.origin,
            SiteKind::Sahagin(sg) => sg.origin,
            SiteKind::VampireCastle(vc) => vc.origin,
            SiteKind::GliderCourse(gc) => gc.origin,
            SiteKind::Myrmidon(my) => my.origin,
        }
    }

    pub fn spawn_rules(&self, wpos: Vec2<i32>) -> SpawnRules {
        match &self.kind {
            SiteKind::Settlement(s) => s.spawn_rules(wpos),
            SiteKind::Castle(c) => c.spawn_rules(wpos),
            SiteKind::Refactor(s) => s.spawn_rules(wpos),
            SiteKind::CliffTown(ct) => ct.spawn_rules(wpos),
            SiteKind::SavannahTown(st) => st.spawn_rules(wpos),
            SiteKind::CoastalTown(ct) => ct.spawn_rules(wpos),
            SiteKind::PirateHideout(ph) => ph.spawn_rules(wpos),
            SiteKind::JungleRuin(jr) => jr.spawn_rules(wpos),
            SiteKind::RockCircle(rc) => rc.spawn_rules(wpos),
            SiteKind::TrollCave(tc) => tc.spawn_rules(wpos),
            SiteKind::Camp(cp) => cp.spawn_rules(wpos),
            SiteKind::DesertCity(dc) => dc.spawn_rules(wpos),
            SiteKind::ChapelSite(p) => p.spawn_rules(wpos),
            SiteKind::DwarvenMine(dm) => dm.spawn_rules(wpos),
            SiteKind::Terracotta(tr) => tr.spawn_rules(wpos),
            SiteKind::Tree(t) => t.spawn_rules(wpos),
            SiteKind::GiantTree(gt) => gt.spawn_rules(wpos),
            SiteKind::Gnarling(g) => g.spawn_rules(wpos),
            SiteKind::Bridge(b) => b.spawn_rules(wpos),
            SiteKind::Adlet(ad) => ad.spawn_rules(wpos),
            SiteKind::Haniwa(ha) => ha.spawn_rules(wpos),
            SiteKind::Cultist(cl) => cl.spawn_rules(wpos),
            SiteKind::Sahagin(sg) => sg.spawn_rules(wpos),
            SiteKind::VampireCastle(vc) => vc.spawn_rules(wpos),
            SiteKind::GliderCourse(gc) => gc.spawn_rules(wpos),
            SiteKind::Myrmidon(my) => my.spawn_rules(wpos),
        }
    }

    pub fn name(&self) -> &str {
        match &self.kind {
            SiteKind::Settlement(s) => s.name(),
            SiteKind::Castle(c) => c.name(),
            SiteKind::Refactor(s) => s.name(),
            SiteKind::CliffTown(ct) => ct.name(),
            SiteKind::SavannahTown(st) => st.name(),
            SiteKind::CoastalTown(ct) => ct.name(),
            SiteKind::PirateHideout(ph) => ph.name(),
            SiteKind::JungleRuin(jr) => jr.name(),
            SiteKind::RockCircle(rc) => rc.name(),
            SiteKind::TrollCave(tc) => tc.name(),
            SiteKind::Camp(cp) => cp.name(),
            SiteKind::DesertCity(dc) => dc.name(),
            SiteKind::ChapelSite(p) => p.name(),
            SiteKind::Terracotta(tr) => tr.name(),
            SiteKind::DwarvenMine(dm) => dm.name(),
            SiteKind::Tree(_) => "Giant Tree",
            SiteKind::GiantTree(gt) => gt.name(),
            SiteKind::Gnarling(g) => g.name(),
            SiteKind::Bridge(b) => b.name(),
            SiteKind::Adlet(ad) => ad.name(),
            SiteKind::Haniwa(ha) => ha.name(),
            SiteKind::Cultist(cl) => cl.name(),
            SiteKind::Sahagin(sg) => sg.name(),
            SiteKind::VampireCastle(vc) => vc.name(),
            SiteKind::GliderCourse(gc) => gc.name(),
            SiteKind::Myrmidon(my) => my.name(),
        }
    }

    pub fn trade_information(
        &self,
        site_id: common::trade::SiteId,
    ) -> Option<common::trade::SiteInformation> {
        match &self.kind {
            SiteKind::Settlement(_)
            | SiteKind::Refactor(_)
            | SiteKind::CliffTown(_)
            | SiteKind::SavannahTown(_)
            | SiteKind::CoastalTown(_)
            | SiteKind::DesertCity(_) => Some(common::trade::SiteInformation {
                id: site_id,
                unconsumed_stock: self.economy.get_available_stock(),
            }),
            _ => None,
        }
    }

    pub fn apply_to(&self, canvas: &mut Canvas, dynamic_rng: &mut impl Rng) {
        let info = canvas.info();
        let get_col = |wpos| info.col(wpos + info.wpos);
        match &self.kind {
            SiteKind::Settlement(s) => s.apply_to(canvas.index, canvas.wpos, get_col, canvas.chunk),
            SiteKind::Castle(c) => c.apply_to(canvas.index, canvas.wpos, get_col, canvas.chunk),
            SiteKind::Refactor(s) => s.render(canvas, dynamic_rng),
            SiteKind::CliffTown(ct) => ct.render(canvas, dynamic_rng),
            SiteKind::SavannahTown(st) => st.render(canvas, dynamic_rng),
            SiteKind::CoastalTown(ct) => ct.render(canvas, dynamic_rng),
            SiteKind::PirateHideout(ph) => ph.render(canvas, dynamic_rng),
            SiteKind::JungleRuin(jr) => jr.render(canvas, dynamic_rng),
            SiteKind::RockCircle(rc) => rc.render(canvas, dynamic_rng),
            SiteKind::TrollCave(tc) => tc.render(canvas, dynamic_rng),
            SiteKind::Camp(cp) => cp.render(canvas, dynamic_rng),
            SiteKind::DesertCity(dc) => dc.render(canvas, dynamic_rng),
            SiteKind::ChapelSite(p) => p.render(canvas, dynamic_rng),
            SiteKind::Terracotta(tr) => tr.render(canvas, dynamic_rng),
            SiteKind::DwarvenMine(dm) => dm.render(canvas, dynamic_rng),
            SiteKind::Tree(t) => t.render(canvas, dynamic_rng),
            SiteKind::GiantTree(gt) => gt.render(canvas, dynamic_rng),
            SiteKind::Gnarling(g) => g.render(canvas, dynamic_rng),
            SiteKind::Bridge(b) => b.render(canvas, dynamic_rng),
            SiteKind::Adlet(ad) => ad.render(canvas, dynamic_rng),
            SiteKind::Haniwa(ha) => ha.render(canvas, dynamic_rng),
            SiteKind::Cultist(cl) => cl.render(canvas, dynamic_rng),
            SiteKind::Sahagin(sg) => sg.render(canvas, dynamic_rng),
            SiteKind::VampireCastle(vc) => vc.render(canvas, dynamic_rng),
            SiteKind::GliderCourse(gc) => gc.render(canvas, dynamic_rng),
            SiteKind::Myrmidon(my) => my.render(canvas, dynamic_rng),
        }
    }

    pub fn apply_supplement<'a>(
        &'a self,
        // NOTE: Used only for dynamic elements like chests and entities!
        dynamic_rng: &mut impl Rng,
        wpos2d: Vec2<i32>,
        get_column: impl FnMut(Vec2<i32>) -> Option<&'a ColumnSample<'a>>,
        supplement: &mut ChunkSupplement,
        site_id: common::trade::SiteId,
        time: Option<&(TimeOfDay, Calendar)>,
    ) {
        match &self.kind {
            SiteKind::Settlement(s) => {
                let economy = self
                    .trade_information(site_id)
                    .expect("Settlement has no economy");
                s.apply_supplement(dynamic_rng, wpos2d, get_column, supplement, economy, time)
            },
            SiteKind::Castle(c) => c.apply_supplement(dynamic_rng, wpos2d, get_column, supplement),
            SiteKind::Refactor(_)
            | SiteKind::CliffTown(_)
            | SiteKind::SavannahTown(_)
            | SiteKind::CoastalTown(_)
            | SiteKind::PirateHideout(_)
            | SiteKind::JungleRuin(_)
            | SiteKind::RockCircle(_)
            | SiteKind::TrollCave(_)
            | SiteKind::Camp(_)
            | SiteKind::DesertCity(_)
            | SiteKind::GliderCourse(_)
            | SiteKind::Tree(_) => {},
            SiteKind::ChapelSite(p) => p.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::Terracotta(tr) => tr.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::DwarvenMine(dm) => dm.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::GiantTree(gt) => gt.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::Gnarling(g) => g.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::Bridge(b) => b.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::Adlet(ad) => ad.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::Haniwa(ha) => ha.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::Cultist(cl) => cl.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::Sahagin(sg) => sg.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::VampireCastle(vc) => vc.apply_supplement(dynamic_rng, wpos2d, supplement),
            SiteKind::Myrmidon(my) => my.apply_supplement(dynamic_rng, wpos2d, supplement),
        }
    }

    pub fn do_economic_simulation(&self) -> bool {
        matches!(
            self.kind,
            SiteKind::Refactor(_)
                | SiteKind::CliffTown(_)
                | SiteKind::SavannahTown(_)
                | SiteKind::CoastalTown(_)
                | SiteKind::DesertCity(_)
                | SiteKind::Settlement(_)
        )
    }

    /// Return the inner site2 site, if this site has one.
    // TODO: Remove all of this when site1 gets removed.
    pub fn site2(&self) -> Option<&site2::Site> {
        match &self.kind {
            SiteKind::Settlement(_) => None,
            SiteKind::Castle(_) => None,
            SiteKind::Refactor(site2) => Some(site2),
            SiteKind::CliffTown(site2) => Some(site2),
            SiteKind::SavannahTown(site2) => Some(site2),
            SiteKind::CoastalTown(site2) => Some(site2),
            SiteKind::PirateHideout(site2) => Some(site2),
            SiteKind::JungleRuin(site2) => Some(site2),
            SiteKind::RockCircle(site2) => Some(site2),
            SiteKind::TrollCave(site2) => Some(site2),
            SiteKind::Camp(site2) => Some(site2),
            SiteKind::Tree(_) => None,
            SiteKind::DesertCity(site2) => Some(site2),
            SiteKind::ChapelSite(site2) => Some(site2),
            SiteKind::DwarvenMine(site2) => Some(site2),
            SiteKind::Terracotta(site2) => Some(site2),
            SiteKind::GiantTree(site2) => Some(site2),
            SiteKind::Gnarling(site2) => Some(site2),
            SiteKind::Bridge(site2) => Some(site2),
            SiteKind::Adlet(site2) => Some(site2),
            SiteKind::Haniwa(site2) => Some(site2),
            SiteKind::Cultist(site2) => Some(site2),
            SiteKind::Sahagin(site2) => Some(site2),
            SiteKind::VampireCastle(site2) => Some(site2),
            SiteKind::GliderCourse(site2) => Some(site2),
            SiteKind::Myrmidon(site2) => Some(site2),
        }
    }
}

impl SiteKind {
    pub fn convert_to_meta(&self) -> Option<SiteKindMeta> {
        match self {
            SiteKind::Refactor(_) | SiteKind::Settlement(_) => {
                Some(SiteKindMeta::Settlement(SettlementKindMeta::Default))
            },
            SiteKind::CliffTown(_) => Some(SiteKindMeta::Settlement(SettlementKindMeta::CliffTown)),
            SiteKind::SavannahTown(_) => {
                Some(SiteKindMeta::Settlement(SettlementKindMeta::SavannahTown))
            },
            SiteKind::CoastalTown(_) => {
                Some(SiteKindMeta::Settlement(SettlementKindMeta::CoastalTown))
            },
            SiteKind::DesertCity(_) => {
                Some(SiteKindMeta::Settlement(SettlementKindMeta::DesertCity))
            },
            SiteKind::Gnarling(_) => Some(SiteKindMeta::Dungeon(DungeonKindMeta::Gnarling)),
            SiteKind::Adlet(_) => Some(SiteKindMeta::Dungeon(DungeonKindMeta::Adlet)),
            SiteKind::Terracotta(_) => Some(SiteKindMeta::Dungeon(DungeonKindMeta::Terracotta)),
            SiteKind::Haniwa(_) => Some(SiteKindMeta::Dungeon(DungeonKindMeta::Haniwa)),
            SiteKind::Myrmidon(_) => Some(SiteKindMeta::Dungeon(DungeonKindMeta::Myrmidon)),
            SiteKind::DwarvenMine(_) => Some(SiteKindMeta::Dungeon(DungeonKindMeta::DwarvenMine)),
            SiteKind::ChapelSite(_) => Some(SiteKindMeta::Dungeon(DungeonKindMeta::SeaChapel)),
            SiteKind::Cultist(_) => Some(SiteKindMeta::Dungeon(DungeonKindMeta::Cultist)),
            SiteKind::Sahagin(_) => Some(SiteKindMeta::Dungeon(DungeonKindMeta::Sahagin)),
            SiteKind::VampireCastle(_) => {
                Some(SiteKindMeta::Dungeon(DungeonKindMeta::VampireCastle))
            },

            _ => None,
        }
    }
}
