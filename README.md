# agent-harmonic-field

**Harmonic fields as shared context spaces for multi-agent coordination.**

When agents share a harmonic field, they agree on the same tonal center, available pitches, and functional roles — enabling coherent collective action. This crate models musical harmony as a framework for shared context in multi-agent systems.

## Core Concept

In music, a **harmonic field** defines the "rules of the game" — what notes are available, what relationships are meaningful, and how tension and resolution work. When musicians play "in the same key," they're operating in a shared context space.

For multi-agent systems:
- **Harmonic field** → a shared context that defines what's "in bounds"
- **Key signature** → the contract agents agree to follow
- **Chord function** → the role each agent plays (leader, supporter, tension-builder)
- **Modulation** → changing the shared context mid-session
- **Chromatic alteration** → intentionally breaking rules for expressiveness

## Key Types

### `HarmonicField`
A shared tonal context. Defines a tonal center (root) and a set of available pitch classes (scale). Includes constructors for major, minor, harmonic minor, dorian, whole-tone, and chromatic fields, plus custom fields.

```rust
let c_major = HarmonicField::major(0);  // C major: C D E F G A B
let d_minor = HarmonicField::minor(2);  // D minor: D E F G A Bb C
let wt = HarmonicField::whole_tone(0);  // Whole tone: C D E F# G# A#
```

Fields support membership queries, complement (what's outside the field), density, and interval computation.

### `KeySignature`
The formal "contract" of a key. Wraps a harmonic field with accidental count and relative key information. Provides helpers for identifying diatonic vs. chromatic pitches, finding the dominant/subdominant/leading tone, and transposing pitches into key.

### `ChordFunction`
The functional role of a chord/agent within a key:
- **Tonic** — home base, stable, at rest (tension: 0.0)
- **Subdominant** — preparing, moving away (tension: 0.35)
- **Dominant** — maximum tension, pulling home (tension: 0.85)
- **Leading tone** — intense pull toward resolution (tension: 0.95)
- **Chromatic** — outside the key, colorful (tension: 0.7)

Each function knows its resolution tendency — where it wants to go next.

### `Modulation`
Changing the shared context from one key to another. Models:
- **Direct modulation** — instant context switch
- **Common tone modulation** — shared element bridges the transition
- **Pivot chord modulation** — a chord that exists in both keys

Includes smoothness scoring and distance measurement (how many accidentals different).

### `FieldCoherence`
Measures how well agents stay within a shared harmonic field. Tracks pitch class output, computes diatonic/chromatic ratios, generates usage histograms, and checks whether the tonal center is the strongest pitch.

### `ChromaticAlteration`
Intentionally breaking the field's rules for color and expressiveness. Models different purposes (passing tones, applied chords, modal mixture, etc.) with color intensity scoring.

## Usage

```rust
use agent_harmonic_field::*;

// Create a shared context
let field = HarmonicField::major(0);  // C major
let key = KeySignature::c_major();

// Check chord functions
let func = ChordFunction::from_root(7, &key); // G → Dominant
println!("{} has tension {:.2}", func, func.tension());

// Measure coherence
let mut coherence = FieldCoherence::new(field.clone());
coherence.record_all(&[0, 2, 4, 5, 7, 9, 11]);
println!("Coherence: {:.1}%", coherence.diatonic_ratio() * 100.0);

// Modulate to a new key
let g_major = KeySignature::g_major();
let modu = Modulation::via_common_tone(key, g_major, 7);
println!("Smoothness: {:.2}", modu.smoothness());
```

## Design Philosophy

Harmony theory is fundamentally about **shared context management**:

1. A **key** is a bounded context — agents know what's "inside" and "outside"
2. **Chord functions** are role assignments — every agent has a job
3. **Modulation** is context switching — with smooth and abrupt variants
4. **Coherence** measures alignment — how well agents share context
5. **Chromatic alteration** is controlled rule-breaking — expressiveness within structure

These concepts apply directly to any system where multiple agents need to operate within shared constraints while maintaining individuality.

## License

MIT
