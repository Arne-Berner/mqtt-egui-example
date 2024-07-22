# Nutzung
Ihr könnt das Feld zwar per Drag bedienen, aber derzeit funktioniert es durch die eingebaute Pause einfach besser über die direkte Eingabe. (Es war vorher ein Slider, aber das gab keinen Mehrwert, außer dass es ständig aktualisiert hat und das Feld nicht richtig genutzt werden konnte)

# Einstellung
Derzeit müsst ihr die folgenden Variablen anpassen:
```rust
let url = "THEACTUALURL";
let topic = "THEACTUALTOPIC";
```
U.u. kommt bald ein update, was die Wartezeiten zwischen den Updates verkürzt.

# Installieren
Es sollte nichts weiter als Rust installiert werden müssen. Falls doch, sagt bitte bescheid.

# Kompilieren und starten
Einfach in die Konsole:
```cmd
cargo run
```
eingeben. Für die beste Performance geht auch
```cmd
cargo run --release
```
Nur das erste mal kompilieren dauert lange, danach sollte es recht flott gehen.
