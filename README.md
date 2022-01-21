# **Rust-Projektarbeit-Reguläre-Ausdrücke**
In diesem Github Repository verwaltet David B. (beda1044) den Fortschritt seiner Projektarbeit zum Thema Rust und Reguläre Ausdrücke sowie deren Umwandlung in endliche Automaten.

Informationen über Rust habe ich hierbei aus einer Vielzahl and Tutorials und der [offiziellen Rust Dokumentation](https://doc.rust-lang.org/) entnommen.

Der [Leitfaden zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/labor.html#(10)) wurde dabei vom betreuenden Professor zur Verfügung gestellt. 

Ein Grundlegendes Verständis der Thematik 'Reguläre Ausdrücke und deren Bildungsgesetze' sowie 'Endliche Automaten' ist zur Nachvollziehbarkeit dieser Arbeit unumgänglich. 

# **Verwendete Software und Entwicklungsumgebung**
Der Code für diese Projektarbeit wurde mithilfe von [Visual Studio Code](https://code.visualstudio.com/) entwickelt. Hierbei kamen außerdem die [standard Erweiterung](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) für Rust sowie [Rust and Friends](https://marketplace.visualstudio.com/items?itemName=nyxiative.rust-and-friends) zum einsatz. Für die Arbeit mit Github wurde zusätzlich noch [TortoiseGit](https://tortoisegit.org/) verwendet.

# **Aufgabenteil 1 - Reguläre Ausdrücke**
# Konstruktion von Regulären Ausdrücken
Aufgabe war es, die aus der theoretischen Informatik bekannten Regulären Ausdrücke generieren zu können. Hierzu wurde das Enum ***Exp*** mit den Variants **Eps**, **Phi**, **Char**, **Alt**, **Conc** und ***Star*** verwendet.

``` 
pub enum Exp {
    Eps{

    },
    Phi{

    },
    Char {
        val: char,
    },
    Alt {
        left: Box<Exp>,
        right: Box<Exp>,
    },
    Conc {
        left: Box<Exp>,
        right: Box<Exp>, 
    },
    Star{
        obj: Box<Exp>,
    }
}
```
Die Variants stehen jeweils für:

eps - "Epsilon" den leeren String

phi - "Phi" die leere Sprache

c - das Zeichen c

r1 | r2 - Die Alternative zwischen r1 und r2

r1 r2 - Die Konkatenation von r1 mit r2

r* - Die Kleenesche Hülle um r


Mit Hilfe dieses Enums lassen sich also alle beliebeigen regulären Ausdrücke konstruieren, indem man die einzelnen Elemente ineinander verschachtelt. Dies wird im Code durch rekursive Aufrufe umgesetzt. Die drei Varianten **Alt**, **Conc** und **Star** haben hierfür die Argumente **left**, **right** und **obj** die vom typ __Box&lt;Exp>__ sind und somit wiederum eine der 6 Variants sein können. __Box&lt;Exp>__ statt einem einfachen __Exp__ zu verwenden, ermöglicht den rekursiven Aufruf der einzelnen Variants. Die rekursion endet schließlich bei der Wahl einer der 3 Variants **Eps**, **Phi** und **Char** die keine weitere Schachtelung zulassen.

Die Zeichen des Regulären Ausdrucks werden im Value Argument **val** der Variant **Char** gespeichert. Hier kann ein beliebiger Character gewählt werden.

## **Beispiele**
Den Ausdruck __(a | b)__ bildet man so:
```
let re1 = Box::new(Exp::Alt{
    left: Box::new(Exp::Char{val : 'a'}),
    right: Box::new(Exp::Char{val : 'b'}) 
});
``` 
<br>

Den Ausdruck __a (b | c)*__ bildet man so:
```
let re2 = Box::new(Exp::Conc{
    left: Box::new(Exp::Char{val : 'a'}),
    right: Box::new(Exp::Star{
        obj: Box::new(Exp::Alt{
            left: Box::new(Exp::Char{val : 'a'}),
            right: Box::new(Exp::Char{val : 'b'}) 
        })
    }) 
});
```

<br>

Den Ausdruck __eps ((a*)* (phi | b))__ bildet man so:
```
let re3 = Box::new(Exp::Conc{
    left: Box::new(Exp::Eps{}), 
    right: Box::new(Exp::Conc{
        left: Box::new(Exp::Star{
            obj: Box::new(Exp::Star{
                obj: Box::new(Exp::Char{val : 'a'})
            }) 
        }),
        right: Box::new(Exp::Alt{
            left: Box::new(Exp::Phi{}) , 
            right: Box::new(Exp::Char{val : 'b'}) 
        })
    }) 
});
```


Hierbei ist zu beachten, dass dieser reguläre Ausdruck noch nicht in seiner vereinfachten Form __(a*) b__ vorliegt. Dies führt zum nächsten Thema dieser Projektarbeit.

<br>

# Vereinfachung von Regulären Ausdrücken
Manche Regulären Ausdrücke lassen sich logisch vereinfachen, wodurch eine übersichtlichere Schreibweise ermöglicht wird. Hierbei bleibt die Bedeutung des Ausdrucks erhalten. Die Regeln nach denen man Reguläre Ausdrücke vereinfachen kann lauten wie folgt:
```
1. eps r ==> r

2. r1 r2 ==> phi falls L(r1)={} oder L(r2)={}

3. r* ==> eps falls L(r)={}

4. (r*)* ==> r*

5. r | r ==> r

6. r1 | r2 ==> r2 falls L(r1)={}

7. r1 | r2 ==> r1 falls L(r2)={}
```
Hiebei ist zu beachten, dass es sich bei ```L(r1)={}``` um einen Regulären Ausdruck handelt, der die leere Sprache bildet. 

Die Vereinfachung von Regulären Ausdrücken wird in diesem Projekt durch die Funktion ```simplify(x : &Exp) -> Exp {}``` geregelt, die einen Regulären Ausdruck vom Typ **Exp** erhält und diesen als vereinfachten Ausdruck zurück gibt, der ebenfalls als Typ **Exp** gespeichert ist.
Hierbei wird rekursiv gearbeitet und  geprüft ob beim aktuellen Element des Ausdrucks eine der 7 Vereinfachungsregeln Anwendung findet. Falls es keine möglichen Vereinfachungen gibt, wird der Ausdruck unverändert zurückgegeben.

## **Beispiele**
Der Ausdruck __a (b|b)__ lässt sich wegen Regel 6 vereinfachen zu __a b__. Im Code ist das folgendermaßen umsetzbar:
```
let re1 = Box::new(Exp::Conc{
    left: Box::new(Exp::Char{val : 'a'}),
    right: Box::new(Exp::Alt{
        left: Box::new(Exp::Char{val : 'b'}),
        right: Box::new(Exp::Char{val : 'b'}) 
    }) 
});
println!("This is the simplified Version of re1: {:?}", simplify(&re1));
```
Dieser Code hätte die Ausgabe __This is the simplified Version of re1: Conc { left: Char { val: 'a' }, right: Char { val: 'b' } }__ zur Folge.

<br>

Der Ausdruck __((a)*) *__ lässt sich wegen Regel 4 vereinfachen zu __a *__. Im Code ist das folgendermaßen umsetzbar:
```
let re2 = Box::new(Exp::Star{
    obj: Box::new(Exp::Star{
        obj: Box::new(Exp::Char{val : 'a'})
    }) 
});
println!("This is the simplified Version of re2: {:?}", simplify(&re2));
```
Dieser Code hätte die Ausgabe __This is the simplified Version of re2: Star { obj: Char { val: 'a' } }__ zur Folge.

<br>

Den Ausdruck __eps ((a*)* (phi | b))__ lässt sich wegen einer Kombination aus mehreren Regeln zu __(a*) b__ vereinfachen. Im Code ist dies folgendermaßen umsetzbar:
```
let re3 = Box::new(Exp::Conc{
    left: Box::new(Exp::Eps{}), 
    right: Box::new(Exp::Conc{
        left: Box::new(Exp::Star{
            obj: Box::new(Exp::Star{
                obj: Box::new(Exp::Char{val : 'a'})
            }) 
        }),
        right: Box::new(Exp::Alt{
            left: Box::new(Exp::Phi{}) , 
            right: Box::new(Exp::Char{val : 'b'}) 
        })
    }) 
});
println!("This is the simplified Version of re3: {:?}", simplify(&re3));
```
Dieser Code hätte die Ausgabe __This is the simplified Version of re3: Conc { left: Star { obj: Char { val: 'a' } }, right: Char { val: 'b' } }__ zur Folge.

<br>

Da diese Schreibweise aber immernoch sehr unlesbar ist, wurde zusätzlich eine PrettyPrint Funktion implementiert, die den Ausdruck ordentlich lesbar zurück gibt. Um diese Funktion soll es als nächstes gehen.

<br>

# Printen von Regulären Ausdrücken

Die lesbare Darstellung von Regulären Ausdrücken wird in diesem Code mit der Funktion ```pretty(x : &Exp) -> String {} ``` umgesetzt die ebenfalls einen Ausdruck vom Typ **Exp** erhält und diesen als ein String zurück gibt. 

Im Grunde arbeitet diese Funktion ähnlich wie auch ```simplify(x : &Exp) -> Exp {}```, indem sie rekursiv durch den Ausdruck schreitet und das aktuelle element, statt es zu vereinfachen, in einen String umwandelt. Bei Elementen, die wiederum ein **left**, **right** oder **obj** enthalten, wird eine tiefere Schachtelung aufgerufen.

- Alternative``` "(".to_string() + &pretty(&left) + &"|".to_string()+ &pretty(&right) + &")".to_string();```
- Concatenation: ```"(".to_string() + &pretty(&left) + &pretty(&right) + &")".to_string();```
- Star: ```"(".to_string() + &pretty(&obj) + &"*)".to_string();```

Die Pretty Funktion kann direkt auf einen selbst erstellten Ausdruck angewandt werden, oder aber den Return-Wert der Simplify Funktion als Parameter übergeben bekommen.

## **Beispiele**

Es wird erneut der Ausdruck __eps ((a*)* (phi | b))__ betrachtet, wobei dieser zuerst in seiner ursprünglichen Form und anschließend in seiner vereinfachten Form __(a*) b__ auf der Konsole ausgegeben werden soll. 

```
    let re3 = Box::new(Exp::Conc{
        left: Box::new(Exp::Eps{}), 
        right: Box::new(Exp::Conc{
            left: Box::new(Exp::Star{
                obj: Box::new(Exp::Star{
                    obj: Box::new(Exp::Char{val : 'a'})
                }) 
            }),
            right: Box::new(Exp::Alt{
                left: Box::new(Exp::Phi{}) , 
                right: Box::new(Exp::Char{val : 'b'}) 
            })
        }) 
    });
    println!("This is the pretty print of the original re3: {:?}", pretty(&re3));
    println!("This is the pretty print of the simplified re3: {:?}", pretty(&simplify(&re3)));
```
Dieser Code hätte die Ausgabe __This is the pretty print of the original re3: "((((a*)*)(phi|b)))"__ sowie __This is the pretty print of the simplified re3: "((a*)b)"__ zur Folge, wobei die äußeren Klammern in beiden Fällen nicht nötig wären.

# **Aufgabenteil 2 - Endliche Automaten**

# Konstruktion von Endlichen Automaten

Ein Endlicher Automat besteht in diesem Anwendungsfall aus einem Startzustand: **initial_state**, einer Reihe an Zustandsübergängen: **transitions** und einem Endzustand: **final_state**. Die Zustandsübergänge verfügen hierbei über einen Ausgangszustand: **from** und einen Zielzustand: **to**, sowie das zu verarbeitende Zeichen: **char**. Sollte es sich bei dem Zustandübergang um einen spontanen Übergang handeln, so wird der leere Character als ersatz für das leere Wort **Epsilon** verwendet. 

Die besagten Automaten sind in diesem Projekt als **NFA** folgendermaßen implementiert:
```
pub struct NFA{
    pub transitions: Vec<Transition>,
    pub initial_state: i32,
    pub final_state: i32,
}
```
Zustandübergänge werden wiederum so abgebildet:
```
pub struct Transition{
    pub from: i32,
    pub char: char,
    pub to: i32,
}
```
# Transformation von Regulären Ausdrücken in Endliche Automaten

In diesem Abschnitt geht es darum, die Regulären Ausdrücke aus Aufgabenteil 1 in eben erklärte Endliche Automaten umzuwandeln. Hierbei wird nach den Regeln des [Thompson NFA Algorithmus](https://en.wikipedia.org/wiki/Thompson%27s_construction) vorgegangen, der für alle 6 möglichen Elemente eines Regulären Ausdrucks Bildungsgesetze für Automaten enthält. Um den Umfang dieser Ausarbeitung in Grenzen zu halten, wird die Vertrautheit der Lesenden mit eben diesen Bildungsgesetzen als bekannt vorrausgesetzt.

Die Transformation wird in diesem Projekt von dem Struct **TransformWorker** umgesetzt, der über eine namensgebende Funktion ```fn transform_worker(self, re: &Exp)->Box<NFA>``` verfügt. Sie erhällt einen Regulären Ausdruck vom Typ **Exp** und gibt zurück .....




# **Aufgabenteil 3 - Ausführen von Automaten**
Dieser Teil der Ausarbeitung existiert noch nicht, da die Arbeit am Code noch nicht begonnen hat.