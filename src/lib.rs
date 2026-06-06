//! # agent-harmonic-field
//!
//! Harmonic fields as shared context spaces for multi-agent coordination.
//! When agents share a harmonic field, they agree on the same tonal center,
//! available pitches, and functional roles — enabling coherent collective action.

use std::fmt;

/// Number of pitch classes in the chromatic system.
const CHROMATIC_COUNT: usize = 12;

/// A pitch class (0–11) in the chromatic system.
pub type PitchClass = u8;

/// A harmonic field: a shared tonal context that defines what notes and
/// relationships agents can use. Think of it as a "room" where agents
/// operate under shared rules.
#[derive(Debug, Clone, PartialEq)]
pub struct HarmonicField {
    /// The tonal center (root pitch class, 0–11).
    pub tonal_center: PitchClass,
    /// The scale/mode as a set of pitch classes relative to the tonal center.
    pub scale: Vec<PitchClass>,
    /// Human-readable name.
    pub name: String,
}

impl HarmonicField {
    /// Create a major key harmonic field.
    pub fn major(tonic: PitchClass) -> Self {
        let intervals = vec![0, 2, 4, 5, 7, 9, 11];
        let scale: Vec<PitchClass> = intervals
            .iter()
            .map(|&i| (tonic + i) % 12)
            .collect();
        HarmonicField {
            tonal_center: tonic,
            scale,
            name: format!("{:?} Major", NoteName::from_pc(tonic)),
        }
    }

    /// Create a minor key harmonic field (natural minor).
    pub fn minor(tonic: PitchClass) -> Self {
        let intervals = vec![0, 2, 3, 5, 7, 8, 10];
        let scale: Vec<PitchClass> = intervals
            .iter()
            .map(|&i| (tonic + i) % 12)
            .collect();
        HarmonicField {
            tonal_center: tonic,
            scale,
            name: format!("{:?} Minor", NoteName::from_pc(tonic)),
        }
    }

    /// Create a harmonic minor harmonic field.
    pub fn harmonic_minor(tonic: PitchClass) -> Self {
        let intervals = vec![0, 2, 3, 5, 7, 8, 11];
        let scale: Vec<PitchClass> = intervals
            .iter()
            .map(|&i| (tonic + i) % 12)
            .collect();
        HarmonicField {
            tonal_center: tonic,
            scale,
            name: format!("{:?} Harmonic Minor", NoteName::from_pc(tonic)),
        }
    }

    /// Create a dorian mode harmonic field.
    pub fn dorian(tonic: PitchClass) -> Self {
        let intervals = vec![0, 2, 3, 5, 7, 9, 10];
        let scale: Vec<PitchClass> = intervals
            .iter()
            .map(|&i| (tonic + i) % 12)
            .collect();
        HarmonicField {
            tonal_center: tonic,
            scale,
            name: format!("{:?} Dorian", NoteName::from_pc(tonic)),
        }
    }

    /// Create a whole-tone harmonic field.
    pub fn whole_tone(tonic: PitchClass) -> Self {
        let intervals = vec![0, 2, 4, 6, 8, 10];
        let scale: Vec<PitchClass> = intervals
            .iter()
            .map(|&i| (tonic + i) % 12)
            .collect();
        HarmonicField {
            tonal_center: tonic,
            scale,
            name: format!("{:?} Whole Tone", NoteName::from_pc(tonic)),
        }
    }

    /// Create a chromatic harmonic field (all 12 notes).
    pub fn chromatic(tonic: PitchClass) -> Self {
        HarmonicField {
            tonal_center: tonic,
            scale: (0..12).collect(),
            name: "Chromatic".into(),
        }
    }

    /// Create a custom harmonic field from a set of pitch classes.
    pub fn custom(tonic: PitchClass, scale: Vec<PitchClass>, name: String) -> Self {
        HarmonicField {
            tonal_center: tonic,
            scale,
            name,
        }
    }

    /// Whether a pitch class belongs to this field.
    pub fn contains(&self, pc: PitchClass) -> bool {
        self.scale.contains(&pc)
    }

    /// The available pitch classes in this field.
    pub fn available_notes(&self) -> &[PitchClass] {
        &self.scale
    }

    /// Number of distinct pitch classes.
    pub fn cardinality(&self) -> usize {
        self.scale.len()
    }

/// Compute the interval between two pitch classes (ascending).
    pub fn interval_between(&self, a: PitchClass, b: PitchClass) -> u8 {
        (b as i32 - a as i32).rem_euclid(12) as u8
    }

    /// All pitch classes NOT in this field (chromatic complement).
    pub fn complement(&self) -> Vec<PitchClass> {
        (0..12).filter(|pc| !self.contains(*pc)).collect()
    }

    /// The "brightness" of the field: ratio of scale degrees to total chromatic.
    /// Higher means more notes available.
    pub fn density(&self) -> f64 {
        self.scale.len() as f64 / 12.0
    }
}

impl fmt::Display for HarmonicField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Note name helper for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteName {
    C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B,
}

impl NoteName {
    fn from_pc(pc: PitchClass) -> NoteName {
        match pc {
            0 => NoteName::C,
            1 => NoteName::Cs,
            2 => NoteName::D,
            3 => NoteName::Ds,
            4 => NoteName::E,
            5 => NoteName::F,
            6 => NoteName::Fs,
            7 => NoteName::G,
            8 => NoteName::Gs,
            9 => NoteName::A,
            10 => NoteName::As,
            _ => NoteName::B,
        }
    }
}


/// A key signature: the "contract" that agents agree to follow.
/// Defines the tonal center, mode, and associated accidentals.
#[derive(Debug, Clone, PartialEq)]
pub struct KeySignature {
    /// The key's harmonic field.
    pub field: HarmonicField,
    /// Number of sharps (positive) or flats (negative).
    pub accidentals: i32,
    /// The relative major/minor key (if applicable).
    pub relative_key: Option<Box<KeySignature>>,
}

impl KeySignature {
    /// Create a C Major key signature.
    pub fn c_major() -> Self {
        KeySignature {
            field: HarmonicField::major(0),
            accidentals: 0,
            relative_key: Some(Box::new(KeySignature {
                field: HarmonicField::minor(9),
                accidentals: 0,
                relative_key: None,
            })),
        }
    }

    /// Create a G Major key signature.
    pub fn g_major() -> Self {
        KeySignature {
            field: HarmonicField::major(7),
            accidentals: 1,
            relative_key: Some(Box::new(KeySignature {
                field: HarmonicField::minor(4),
                accidentals: 1,
                relative_key: None,
            })),
        }
    }

    /// Create a key signature from a harmonic field.
    pub fn from_field(field: HarmonicField, accidentals: i32) -> Self {
        KeySignature {
            field,
            accidentals,
            relative_key: None,
        }
    }

    /// Whether a pitch class is "in key".
    pub fn is_diatonic(&self, pc: PitchClass) -> bool {
        self.field.contains(pc)
    }

    /// Whether a pitch class is chromatic (outside the key).
    pub fn is_chromatic(&self, pc: PitchClass) -> bool {
        !self.field.contains(pc)
    }

    /// The leading tone (7th scale degree, one semitone below tonic).
    pub fn leading_tone(&self) -> PitchClass {
        (self.field.tonal_center as i32 + 11) as u8 % 12
    }

    /// The dominant (5th scale degree).
    pub fn dominant(&self) -> PitchClass {
        (self.field.tonal_center as i32 + 7) as u8 % 12
    }

    /// The subdominant (4th scale degree).
    pub fn subdominant(&self) -> PitchClass {
        (self.field.tonal_center as i32 + 5) as u8 % 12
    }

    /// The mediant (3rd scale degree).
    pub fn mediant(&self) -> PitchClass {
        (self.field.tonal_center as i32 + (if self.field.scale.contains(&((self.field.tonal_center + 4) % 12)) { 4 } else { 3 })) as u8 % 12
    }

    /// Transpose a pitch class into this key's range (nearest diatonic note).
    pub fn nearest_diatonic(&self, pc: PitchClass) -> PitchClass {
        if self.is_diatonic(pc) {
            return pc;
        }
        // Find the closest diatonic pitch class
        let mut best = pc;
        let mut best_dist = 12;
        for &d in &self.field.scale {
            let dist = (d as i32 - pc as i32).abs().min((d as i32 - pc as i32 + 12).abs().min((d as i32 - pc as i32 - 12).abs()));
            if dist < best_dist {
                best_dist = dist;
                best = d;
            }
        }
        best
    }
}

impl fmt::Display for KeySignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let acc = if self.accidentals > 0 {
            format!("{}#", self.accidentals)
        } else if self.accidentals < 0 {
            format!("{}b", -self.accidentals)
        } else {
            "no accidentals".into()
        };
        write!(f, "Key: {} ({})", self.field.name, acc)
    }
}

/// The functional role of a chord within a harmonic field.
/// Agents take on these roles to coordinate their harmonic responsibilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChordFunction {
    /// Tonic: the home base, stable, at rest.
    Tonic,
    /// Subdominant: moving away from home, preparing.
    Subdominant,
    /// Dominant: tension, pulling toward tonic.
    Dominant,
    /// Supertonic: pre-subdominant, often leads to dominant.
    Supertonic,
    /// Mediant: transitional, ambiguous.
    Mediant,
    /// Submediant: relative minor/major, often deceptive.
    Submediant,
    /// Leading tone: intense pull toward tonic.
    LeadingTone,
    /// Chromatic: outside the key, colorful.
    Chromatic,
}

impl ChordFunction {
    /// Determine the function of a chord root within a key.
    pub fn from_root(root: PitchClass, key: &KeySignature) -> ChordFunction {
        let tc = key.field.tonal_center;
        let interval = (root as i32 - tc as i32).rem_euclid(12) as u8;

        match interval {
            0 => ChordFunction::Tonic,
            2 => ChordFunction::Supertonic,
            3 | 4 => ChordFunction::Mediant,
            5 => ChordFunction::Subdominant,
            7 => ChordFunction::Dominant,
            9 => ChordFunction::Submediant,
            11 => ChordFunction::LeadingTone,
            _ => ChordFunction::Chromatic,
        }
    }

    /// Tension level: 0 (stable) to 1.0 (maximum tension).
    pub fn tension(&self) -> f64 {
        match self {
            ChordFunction::Tonic => 0.0,
            ChordFunction::Subdominant => 0.35,
            ChordFunction::Supertonic => 0.45,
            ChordFunction::Mediant => 0.25,
            ChordFunction::Submediant => 0.3,
            ChordFunction::Dominant => 0.85,
            ChordFunction::LeadingTone => 0.95,
            ChordFunction::Chromatic => 0.7,
        }
    }

    /// Resolution tendency: where this function wants to go.
    pub fn resolves_to(&self) -> Vec<ChordFunction> {
        match self {
            ChordFunction::Tonic => vec![], // Home, doesn't need to resolve
            ChordFunction::Subdominant => vec![ChordFunction::Dominant],
            ChordFunction::Dominant => vec![ChordFunction::Tonic],
            ChordFunction::Supertonic => vec![ChordFunction::Dominant],
            ChordFunction::Mediant => vec![ChordFunction::Submediant, ChordFunction::Subdominant],
            ChordFunction::Submediant => vec![ChordFunction::Dominant, ChordFunction::Subdominant],
            ChordFunction::LeadingTone => vec![ChordFunction::Tonic],
            ChordFunction::Chromatic => vec![ChordFunction::Dominant, ChordFunction::Tonic],
        }
    }

    /// Whether this function provides stability.
    pub fn is_stable(&self) -> bool {
        matches!(self, ChordFunction::Tonic)
    }

    /// Whether this function creates tension.
    pub fn is_tense(&self) -> bool {
        matches!(
            self,
            ChordFunction::Dominant | ChordFunction::LeadingTone
        )
    }
}

impl fmt::Display for ChordFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChordFunction::Tonic => write!(f, "Tonic (I)"),
            ChordFunction::Subdominant => write!(f, "Subdominant (IV)"),
            ChordFunction::Dominant => write!(f, "Dominant (V)"),
            ChordFunction::Supertonic => write!(f, "Supertonic (ii)"),
            ChordFunction::Mediant => write!(f, "Mediant (iii)"),
            ChordFunction::Submediant => write!(f, "Submediant (vi)"),
            ChordFunction::LeadingTone => write!(f, "Leading Tone (vii°)"),
            ChordFunction::Chromatic => write!(f, "Chromatic"),
        }
    }
}

/// A modulation: changing the shared harmonic field mid-session.
/// Agents transition from one key/context to another.
#[derive(Debug, Clone, PartialEq)]
pub struct Modulation {
    /// Source key.
    pub from_key: KeySignature,
    /// Target key.
    pub to_key: KeySignature,
    /// The pivot chord or element used to transition.
    pub pivot: Option<PivotElement>,
    /// How gradual the modulation is (0.0 = instant, 1.0 = very gradual).
    pub gradualness: f64,
}

/// The element used as a bridge between two keys during modulation.
#[derive(Debug, Clone, PartialEq)]
pub enum PivotElement {
    /// A pitch class common to both keys.
    CommonTone(PitchClass),
    /// A chord that exists in both keys.
    PivotChord { root: PitchClass, quality: String },
    /// A sequential pattern that shifts the tonal center.
    Sequence,
    /// Direct modulation (no pivot).
    Direct,
}

impl Modulation {
    /// Create a direct modulation (no pivot).
    pub fn direct(from: KeySignature, to: KeySignature) -> Self {
        Modulation {
            from_key: from,
            to_key: to,
            pivot: Some(PivotElement::Direct),
            gradualness: 0.0,
        }
    }

    /// Create a modulation using a common tone.
    pub fn via_common_tone(from: KeySignature, to: KeySignature, tone: PitchClass) -> Self {
        Modulation {
            from_key: from,
            to_key: to,
            pivot: Some(PivotElement::CommonTone(tone)),
            gradualness: 0.5,
        }
    }

    /// Create a pivot chord modulation.
    pub fn via_pivot_chord(
        from: KeySignature,
        to: KeySignature,
        root: PitchClass,
        quality: &str,
    ) -> Self {
        Modulation {
            from_key: from,
            to_key: to,
            pivot: Some(PivotElement::PivotChord {
                root,
                quality: quality.to_string(),
            }),
            gradualness: 0.6,
        }
    }

    /// Find common tones between source and target keys.
    pub fn common_tones(&self) -> Vec<PitchClass> {
        self.from_key
            .field
            .scale
            .iter()
            .filter(|&&pc| self.to_key.field.contains(pc))
            .copied()
            .collect()
    }

    /// Number of common tones.
    pub fn common_tone_count(&self) -> usize {
        self.common_tones().len()
    }

/// How "distant" the modulation is (number of accidentals different).
    pub fn distance(&self) -> u32 {
        (self.to_key.accidentals - self.from_key.accidentals).unsigned_abs()
    }

    /// Whether the modulation is to a closely related key.
    pub fn is_closely_related(&self) -> bool {
        self.distance() <= 1
    }

    /// Smoothness score: how easy the transition is (0.0–1.0).
    pub fn smoothness(&self) -> f64 {
        let common_ratio =
            self.common_tone_count() as f64 / self.from_key.field.scale.len().min(self.to_key.field.scale.len()) as f64;
        let distance_penalty = self.distance() as f64 * 0.1;
        let pivot_bonus = if self.pivot.is_some() { 0.1 } else { 0.0 };
        (common_ratio - distance_penalty + pivot_bonus).clamp(0.0, 1.0)
    }
}

impl fmt::Display for Modulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Modulation: {} → {} (distance: {}, smoothness: {:.2})",
            self.from_key.field.name,
            self.to_key.field.name,
            self.distance(),
            self.smoothness()
        )
    }
}

/// Measures how well agents stay coherent within a shared harmonic field.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldCoherence {
    /// The harmonic field being measured against.
    pub field: HarmonicField,
    /// Pitch classes produced by all agents over time.
    pub agent_output: Vec<PitchClass>,
}

impl FieldCoherence {
    /// Create a coherence measure for a field.
    pub fn new(field: HarmonicField) -> Self {
        FieldCoherence {
            field,
            agent_output: Vec::new(),
        }
    }

    /// Record an agent outputting a pitch class.
    pub fn record(&mut self, pc: PitchClass) {
        self.agent_output.push(pc);
    }

    /// Record multiple pitch classes at once.
    pub fn record_all(&mut self, pcs: &[PitchClass]) {
        self.agent_output.extend(pcs.iter().copied());
    }

    /// Fraction of output that is diatonic (in the field).
    pub fn diatonic_ratio(&self) -> f64 {
        if self.agent_output.is_empty() {
            return 1.0;
        }
        let in_field = self.agent_output.iter().filter(|&&pc| self.field.contains(pc)).count();
        in_field as f64 / self.agent_output.len() as f64
    }

    /// Fraction of output that is chromatic (outside the field).
    pub fn chromatic_ratio(&self) -> f64 {
        1.0 - self.diatonic_ratio()
    }

    /// Overall coherence score (0.0–1.0).
    pub fn score(&self) -> f64 {
        self.diatonic_ratio()
    }

    /// Pitch class usage histogram.
    pub fn usage_histogram(&self) -> [usize; CHROMATIC_COUNT] {
        let mut hist = [0usize; CHROMATIC_COUNT];
        for &pc in &self.agent_output {
            if (pc as usize) < CHROMATIC_COUNT {
                hist[pc as usize] += 1;
            }
        }
        hist
    }

    /// Most commonly used pitch class.
    pub fn most_common(&self) -> Option<PitchClass> {
        let hist = self.usage_histogram();
        hist.iter()
            .enumerate()
            .max_by_key(|&(_, count)| count)
            .filter(|&(_, count)| *count > 0)
            .map(|(pc, _)| pc as PitchClass)
    }

    /// Whether the tonal center is the most common pitch class.
    pub fn tonal_center_is_strong(&self) -> bool {
        self.most_common() == Some(self.field.tonal_center)
    }

    /// Count of unique pitch classes used.
    pub fn unique_count(&self) -> usize {
        let mut seen = [false; CHROMATIC_COUNT];
        for &pc in &self.agent_output {
            if (pc as usize) < CHROMATIC_COUNT {
                seen[pc as usize] = true;
            }
        }
        seen.iter().filter(|&&s| s).count()
    }

    /// Reset the recorded output.
    pub fn reset(&mut self) {
        self.agent_output.clear();
    }
}

impl fmt::Display for FieldCoherence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FieldCoherence({}): {:.1}pct diatonic, {} notes",
            self.field.name,
            self.diatonic_ratio(),
            self.agent_output.len()
        )
    }
}

/// A chromatic alteration: intentionally breaking the field's rules for color.
/// Agents may use chromatic notes for expression while maintaining overall coherence.
#[derive(Debug, Clone, PartialEq)]
pub struct ChromaticAlteration {
    /// The pitch class being altered.
    pub target: PitchClass,
    /// The alteration in semitones (+1 = sharp, -1 = flat).
    pub alteration: i8,
    /// The resulting pitch class.
    pub result: PitchClass,
    /// The purpose of the alteration.
    pub purpose: AlterationPurpose,
}

/// Why an agent makes a chromatic alteration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlterationPurpose {
    /// Passing tone: connects two diatonic notes smoothly.
    Passing,
    /// Neighbor tone: decorates a diatonic note.
    Neighbor,
    /// Applied chord: borrows from another key temporarily.
    AppliedChord,
    /// Modal mixture: borrows from parallel major/minor.
    ModalMixture,
    /// Chromatic mediant: moves to a chord a third away.
    ChromaticMediant,
    /// Augmented sixth: special voice-leading chord.
    AugmentedSixth,
    /// Expressive color: just because it sounds good.
    Expressive,
}

impl ChromaticAlteration {
    /// Create a chromatic alteration.
    pub fn new(target: PitchClass, alteration: i8, purpose: AlterationPurpose) -> Self {
        let result = ((target as i32 + alteration as i32).rem_euclid(12)) as u8;
        ChromaticAlteration {
            target,
            alteration,
            result,
            purpose,
        }
    }

    /// Sharpen a note.
    pub fn sharpen(target: PitchClass, purpose: AlterationPurpose) -> Self {
        ChromaticAlteration::new(target, 1, purpose)
    }

    /// Flatten a note.
    pub fn flatten(target: PitchClass, purpose: AlterationPurpose) -> Self {
        ChromaticAlteration::new(target, -1, purpose)
    }

    /// Whether the alteration creates a pitch outside the given field.
    pub fn is_chromatic_in(&self, field: &HarmonicField) -> bool {
        !field.contains(self.result)
    }

    /// Color intensity: how "foreign" this note is in the given field.
    /// Returns 0.0 if the result is diatonic, higher if more foreign.
    pub fn color_intensity(&self, field: &HarmonicField) -> f64 {
        if field.contains(self.result) {
            0.0
        } else if self.is_chromatic_in(field) {
            match self.purpose {
                AlterationPurpose::Passing => 0.2,
                AlterationPurpose::Neighbor => 0.25,
                AlterationPurpose::AppliedChord => 0.5,
                AlterationPurpose::ModalMixture => 0.4,
                AlterationPurpose::ChromaticMediant => 0.6,
                AlterationPurpose::AugmentedSixth => 0.7,
                AlterationPurpose::Expressive => 0.8,
            }
        } else {
            0.0
        }
    }

    /// Apply the alteration to a sequence of pitch classes.
    pub fn apply_to_sequence(&self, sequence: &mut Vec<PitchClass>) {
        for pc in sequence.iter_mut() {
            if *pc == self.target {
                *pc = self.result;
            }
        }
    }
}

impl fmt::Display for ChromaticAlteration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arrow = if self.alteration > 0 { "♯" } else { "♭" };
        write!(
            f,
            "ChromaticAlteration: {} {} → {} ({:?})",
            self.target, arrow, self.result, self.purpose
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_field() {
        let c_major = HarmonicField::major(0);
        assert_eq!(c_major.tonal_center, 0);
        assert_eq!(c_major.scale.len(), 7);
        assert!(c_major.contains(0)); // C
        assert!(c_major.contains(2)); // D
        assert!(c_major.contains(4)); // E
        assert!(c_major.contains(5)); // F
        assert!(c_major.contains(7)); // G
        assert!(c_major.contains(9)); // A
        assert!(c_major.contains(11)); // B
        assert!(!c_major.contains(1)); // C#
        assert!(!c_major.contains(6)); // F#
    }

    #[test]
    fn test_minor_field() {
        let a_minor = HarmonicField::minor(9);
        assert_eq!(a_minor.tonal_center, 9);
        assert!(a_minor.contains(9)); // A
        assert!(a_minor.contains(11)); // B
        assert!(a_minor.contains(0)); // C
        assert!(!a_minor.contains(1)); // C#
        assert_eq!(a_minor.scale.len(), 7);
    }

    #[test]
    fn test_harmonic_minor() {
        let c_hm = HarmonicField::harmonic_minor(0);
        assert!(c_hm.contains(11)); // B natural (raised 7th)
        assert_eq!(c_hm.scale.len(), 7);
    }

    #[test]
    fn test_dorian_mode() {
        let d_dorian = HarmonicField::dorian(2);
        assert_eq!(d_dorian.tonal_center, 2);
        assert!(d_dorian.contains(2)); // D
        assert!(d_dorian.contains(4)); // E
        assert!(d_dorian.contains(5)); // F (minor third)
        assert!(d_dorian.contains(9)); // A (major sixth)
    }

    #[test]
    fn test_whole_tone() {
        let wt = HarmonicField::whole_tone(0);
        assert_eq!(wt.scale.len(), 6);
        assert!(wt.contains(0));
        assert!(wt.contains(2));
        assert!(wt.contains(4));
        assert!(wt.contains(6));
        assert!(!wt.contains(1));
    }

    #[test]
    fn test_chromatic_field() {
        let chrom = HarmonicField::chromatic(0);
        assert_eq!(chrom.scale.len(), 12);
        assert!(chrom.contains(0));
        assert!(chrom.contains(6));
        assert!(chrom.contains(11));
    }

    #[test]
    fn test_field_density() {
        let major = HarmonicField::major(0);
        assert!((major.density() - 7.0 / 12.0).abs() < 0.01);

        let chrom = HarmonicField::chromatic(0);
        assert!((chrom.density() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_field_complement() {
        let c_major = HarmonicField::major(0);
        let comp = c_major.complement();
        assert_eq!(comp.len(), 5);
        assert!(comp.contains(&1)); // C#
        assert!(comp.contains(&3)); // D#
        assert!(comp.contains(&6)); // F#
        assert!(comp.contains(&8)); // G#
        assert!(comp.contains(&10)); // A#
    }

    #[test]
    fn test_field_display() {
        let c = HarmonicField::major(0);
        assert!(format!("{}", c).contains("Major"));
    }

    #[test]
    fn test_custom_field() {
        let custom = HarmonicField::custom(
            0,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            "Pentatonic+".into(),
        );
        assert_eq!(custom.name, "Pentatonic+");
        assert_eq!(custom.cardinality(), 12);
    }

    #[test]
    fn test_key_signature_c_major() {
        let key = KeySignature::c_major();
        assert_eq!(key.accidentals, 0);
        assert!(key.is_diatonic(0)); // C
        assert!(key.is_diatonic(7)); // G
        assert!(!key.is_diatonic(6)); // F#
        assert!(key.is_chromatic(6));
    }

    #[test]
    fn test_key_signature_dominant() {
        let key = KeySignature::c_major();
        assert_eq!(key.dominant(), 7); // G
        assert_eq!(key.subdominant(), 5); // F
        assert_eq!(key.leading_tone(), 11); // B
    }

    #[test]
    fn test_key_signature_g_major() {
        let key = KeySignature::g_major();
        assert_eq!(key.accidentals, 1);
        assert!(key.is_diatonic(6)); // F# is in G major
        assert!(!key.is_diatonic(5)); // F natural is not
    }

    #[test]
    fn test_key_signature_nearest_diatonic() {
        let key = KeySignature::c_major();
        assert_eq!(key.nearest_diatonic(0), 0); // C is already diatonic
        // C# (1) → C (0) or D (2)
        let nearest = key.nearest_diatonic(1);
        assert!(nearest == 0 || nearest == 2);
    }

    #[test]
    fn test_key_display() {
        let key = KeySignature::c_major();
        let s = format!("{}", key);
        assert!(s.contains("Major"));
        assert!(s.contains("no accidentals"));
    }

    #[test]
    fn test_chord_function_from_root() {
        let key = KeySignature::c_major();
        assert_eq!(ChordFunction::from_root(0, &key), ChordFunction::Tonic);
        assert_eq!(ChordFunction::from_root(7, &key), ChordFunction::Dominant);
        assert_eq!(ChordFunction::from_root(5, &key), ChordFunction::Subdominant);
        assert_eq!(ChordFunction::from_root(2, &key), ChordFunction::Supertonic);
        assert_eq!(ChordFunction::from_root(9, &key), ChordFunction::Submediant);
    }

    #[test]
    fn test_chord_function_tension() {
        assert_eq!(ChordFunction::Tonic.tension(), 0.0);
        assert!(ChordFunction::Dominant.tension() > 0.8);
        assert!(ChordFunction::LeadingTone.tension() > 0.9);
        assert!(ChordFunction::Subdominant.tension() < 0.5);
    }

    #[test]
    fn test_chord_function_resolves_to() {
        assert!(ChordFunction::Dominant.resolves_to().contains(&ChordFunction::Tonic));
        assert!(ChordFunction::Tonic.resolves_to().is_empty());
        assert!(ChordFunction::Subdominant.resolves_to().contains(&ChordFunction::Dominant));
    }

    #[test]
    fn test_chord_function_stability() {
        assert!(ChordFunction::Tonic.is_stable());
        assert!(!ChordFunction::Dominant.is_stable());
        assert!(ChordFunction::Dominant.is_tense());
        assert!(!ChordFunction::Tonic.is_tense());
    }

    #[test]
    fn test_chord_function_display() {
        assert!(format!("{}", ChordFunction::Tonic).contains("I"));
        assert!(format!("{}", ChordFunction::Dominant).contains("V"));
        assert!(format!("{}", ChordFunction::Subdominant).contains("IV"));
    }

    #[test]
    fn test_modulation_direct() {
        let c = KeySignature::c_major();
        let g = KeySignature::g_major();
        let modu = Modulation::direct(c, g);
        assert_eq!(modu.distance(), 1);
        assert!(modu.is_closely_related());
        assert_eq!(modu.gradualness, 0.0);
    }

    #[test]
    fn test_modulation_common_tones() {
        let c = KeySignature::c_major();
        let g = KeySignature::g_major();
        let modu = Modulation::via_common_tone(c, g, 7); // G common tone
        let common = modu.common_tones();
        assert!(!common.is_empty());
        assert!(common.contains(&7)); // G
        assert!(common.contains(&0)); // C
    }

    #[test]
    fn test_modulation_smoothness() {
        let c = KeySignature::c_major();
        let g = KeySignature::g_major();
        let close_mod = Modulation::direct(c.clone(), g);
        assert!(close_mod.smoothness() > 0.3);
    }

    #[test]
    fn test_modulation_distance() {
        let c = KeySignature::c_major();
        let g = KeySignature::g_major();
        let modu = Modulation::direct(c, g);
        assert_eq!(modu.distance(), 1);
    }

    #[test]
    fn test_modulation_display() {
        let c = KeySignature::c_major();
        let g = KeySignature::g_major();
        let modu = Modulation::direct(c, g);
        let s = format!("{}", modu);
        assert!(s.contains("Modulation"));
    }

    #[test]
    fn test_pivot_chord_modulation() {
        let c = KeySignature::c_major();
        let g = KeySignature::g_major();
        let modu = Modulation::via_pivot_chord(c, g, 7, "major");
        assert!(modu.pivot.is_some());
        assert!(modu.gradualness > 0.0);
    }

    #[test]
    fn test_field_coherence_empty() {
        let field = HarmonicField::major(0);
        let coherence = FieldCoherence::new(field);
        assert!((coherence.diatonic_ratio() - 1.0).abs() < 0.001);
        assert!(coherence.agent_output.is_empty());
    }

    #[test]
    fn test_field_coherence_all_diatonic() {
        let field = HarmonicField::major(0);
        let mut coherence = FieldCoherence::new(field);
        coherence.record_all(&[0, 2, 4, 5, 7, 9, 11]);
        assert!((coherence.diatonic_ratio() - 1.0).abs() < 0.001);
        assert!((coherence.chromatic_ratio()).abs() < 0.001);
    }

    #[test]
    fn test_field_coherence_mixed() {
        let field = HarmonicField::major(0);
        let mut coherence = FieldCoherence::new(field);
        coherence.record_all(&[0, 2, 4, 1, 6]); // 3 diatonic, 2 chromatic
        assert!((coherence.diatonic_ratio() - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_field_coherence_histogram() {
        let field = HarmonicField::major(0);
        let mut coherence = FieldCoherence::new(field);
        coherence.record(0);
        coherence.record(0);
        coherence.record(7);
        let hist = coherence.usage_histogram();
        assert_eq!(hist[0], 2);
        assert_eq!(hist[7], 1);
    }

    #[test]
    fn test_field_coherence_most_common() {
        let field = HarmonicField::major(0);
        let mut coherence = FieldCoherence::new(field);
        coherence.record_all(&[0, 0, 0, 7, 7]);
        assert_eq!(coherence.most_common(), Some(0));
    }

    #[test]
    fn test_field_coherence_tonal_center_strength() {
        let field = HarmonicField::major(0);
        let mut coherence = FieldCoherence::new(field);
        coherence.record_all(&[0, 0, 0, 2, 4]);
        assert!(coherence.tonal_center_is_strong());
    }

    #[test]
    fn test_field_coherence_unique_count() {
        let field = HarmonicField::major(0);
        let mut coherence = FieldCoherence::new(field);
        coherence.record_all(&[0, 2, 4, 0, 2, 4, 5]);
        assert_eq!(coherence.unique_count(), 4);
    }

    #[test]
    fn test_field_coherence_reset() {
        let field = HarmonicField::major(0);
        let mut coherence = FieldCoherence::new(field);
        coherence.record_all(&[0, 2, 4]);
        coherence.reset();
        assert!(coherence.agent_output.is_empty());
    }

    #[test]
    fn test_field_coherence_display() {
        let field = HarmonicField::major(0);
        let mut coherence = FieldCoherence::new(field);
        coherence.record_all(&[0, 2, 4]);
        let s = format!("{}", coherence);
        assert!(s.contains("FieldCoherence"));
    }

    #[test]
    fn test_chromatic_alteration_sharpen() {
        let alt = ChromaticAlteration::sharpen(5, AlterationPurpose::AppliedChord);
        assert_eq!(alt.result, 6);
        assert_eq!(alt.alteration, 1);
    }

    #[test]
    fn test_chromatic_alteration_flatten() {
        let alt = ChromaticAlteration::flatten(7, AlterationPurpose::ModalMixture);
        assert_eq!(alt.result, 6);
        assert_eq!(alt.alteration, -1);
    }

    #[test]
    fn test_chromatic_alteration_is_chromatic() {
        let field = HarmonicField::major(0);
        let alt = ChromaticAlteration::sharpen(5, AlterationPurpose::AppliedChord);
        assert!(alt.is_chromatic_in(&field)); // F# is chromatic in C major
    }

    #[test]
    fn test_chromatic_alteration_color_intensity() {
        let field = HarmonicField::major(0);

        let passing = ChromaticAlteration::sharpen(5, AlterationPurpose::Passing);
        assert!(passing.color_intensity(&field) > 0.0);
        assert!(passing.color_intensity(&field) < 0.5);

        let expressive = ChromaticAlteration::sharpen(5, AlterationPurpose::Expressive);
        assert!(expressive.color_intensity(&field) > passing.color_intensity(&field));
    }

    #[test]
    fn test_chromatic_alteration_apply_to_sequence() {
        let alt = ChromaticAlteration::sharpen(5, AlterationPurpose::Passing);
        let mut seq = vec![0, 2, 4, 5, 7];
        alt.apply_to_sequence(&mut seq);
        assert_eq!(seq[3], 6); // F → F#
        assert_eq!(seq[0], 0); // C unchanged
    }

    #[test]
    fn test_chromatic_alteration_display() {
        let alt = ChromaticAlteration::sharpen(5, AlterationPurpose::Passing);
        let s = format!("{}", alt);
        assert!(s.contains("ChromaticAlteration"));
    }

    #[test]
    fn test_interval_between() {
        let field = HarmonicField::major(0);
        assert_eq!(field.interval_between(0, 7), 7);
        assert_eq!(field.interval_between(7, 0), 5);
        assert_eq!(field.interval_between(0, 0), 0);
    }

    #[test]
    fn test_note_name_display() {
        assert_eq!(format!("{:?}", NoteName::from_pc(0)), "C");
        assert_eq!(format!("{:?}", NoteName::from_pc(1)), "Cs");
        assert_eq!(format!("{:?}", NoteName::from_pc(9)), "A");
    }
}
