## Tipo di pianeta

| Tipo | Celle Energetiche | Regola di generazione delle risorse | Razzi (Rockets) | Regole di combinazione delle risorse |
|----|----|----|----|----|
| **A** | `Vec<EnergyCell>` (Molte) | Al massimo una | Al massimo uno | No |
| **B** | `EnergyCell` (Una) | Illimitata (Unbounded) | Zero | Una |
| **C** | `EnergyCell` (Una) | Al massimo una | Al massimo uno | Illimitata (Unbounded) |
| **D** | `Vec<EnergyCell>` (Molte) | Illimitata (Unbounded) | Zero | No |

## Tipo di risorse

### Risorse base

| Risorsa di Base |
|-----------------|
| **Hydrogen**    |
| **Oxygen**      |
| **Carbon**      |
| **Silicon**     |

### Risorse complesse

| Risorsa Complessa | Ricetta Immediata (Input 1 + Input 2) | Risorse Base Totali Necessarie | Tot | Dettaglio della Derivazione |
|----|----|----|----|----|
| **Water** | Hydrogen + Oxygen | <font color="#c00000">1</font> $H$, <font color="#c00000">1 $O$ | 2 | H + O (Entrambe risors</font>e base) |
| **Diamond** | Carbon + Carbon | <font color="#c00000">2 $C$ </font> | 2 | C + C (Entrambe risorse base) |
| **Life** | Water + Carbon | <font color="#c00000">1</font> $H$, <font color="#c00000">1</font> $O$, <font color="#c00000">1</font> $C$ | 3 | **Water** (H+O) + C |
| **Robot** | Silicon + Life | <font color="#c00000">1</font> $Si$, <font color="#c00000">1</font> $H$, <font color="#c00000">1</font> $O$, <font color="#c00000">1</font> $C$ | 4 | Si + **Life** (H+O+C) |
| **Dolphin** | Water + Life | <font color="#c00000">2</font> $H$, <font color="#c00000">2</font> $O$, <font color="#c00000">1</font> $C$ | 5 | **Water** (H+O) + **Life** (H+O+C) |
| **AI-Partner** | Robot + Diamond | <font color="#c00000">1</font> $Si$, <font color="#c00000">1</font> $H$, <font color="#c00000">1</font> $O$, <font color="#c00000">3</font> $C$ | 6 | **Robot** (Si+H+O+C) + **Diamond** (C+C) |

## AI Explorer

- Explorer che cerca di sopravvivere il più possibile
- Scoprire la topologia completa e aggiornata dei pianeti
- Collezionare tutte le risorse possibili
- Massimizzare la quantità di una risorsa specifica (es Ai girlfriend)
- Collezionare tutte le risorse base
- Collezionare tutte le risorse complesse
- Produrre più risorse possibili minimizzando l'attesa
- Consumare completamente tutti i pianeti (energy cell)
- Identificare i nodi critici del grafo, quelli che se venissero distrutti potrebbero dividere il grafo
- Massimizzare il throuput delle risorse complesse senza dover continuamente spostarsi tra i vari pianeti

## Cose da tenere a mente per la scelta

**Ricette Complessa "Collo di Bottiglia"**
**ai del pianeta che privilegia avere sempre missili pronti o preferisce accumulare energy cell per l'explorer**
**robustezza nella concorrenza**
