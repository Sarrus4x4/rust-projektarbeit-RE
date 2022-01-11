# **Rust-Projektarbeit-Reguläre-Ausdrücke**
In diesem Github Repository verwaltet David B. (beda1044) den Fortschritt seiner Projektarbeit zum Thema Rust und Reguläre Ausdrücke sowie deren Umwandlung in endliche Automaten.

Informationen über Rust habe ich hierbei aus einer Vielzahl and Tutorials und der [offiziellen Rust Dokumentation](https://doc.rust-lang.org/) entnommen.

Der [Leitfaden zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/labor.html#(10)) wurde dabei vom betreuenden Professor zur Verfügung gestellt. 

Ein Grundlegendes Verständis der Thematik 'Reguläre Ausdrücke und deren Bildungsgesetze' ist zur Nachvollziehbarkeit dieser Arbeit unumgänglich. 

# **Verwendete Software und Entwicklungsumgebung**
Der Code für diese Projektarbeit wurde mithilfe von [Visual Studio Code](https://code.visualstudio.com/) entwickelt. Hierbei kamen außerdem die [standard Erweiterung](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) für Rust sowie [Rust and Friends](https://marketplace.visualstudio.com/items?itemName=nyxiative.rust-and-friends) zum einsatz. Für die Arbeit mit Github wurde zusätzlich noch [TortoiseGit](https://tortoisegit.org/) verwendet.

# **Aufgabenteil 1 - Reguläre Ausdrücke**
# Erstellen von Regulären Ausdrücken
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


Mit Hilfe dieses Enums lassen sich also alle beliebeigen regulären Ausdrücke konstruieren, indem man die einzelnen Elemente ineinander verschachtelt. Dies wird im Code durch rekursive Aufrufe umgesetzt. Die drei Varianten **Alt**, **Conc** und **Star** haben hierfür die Argumente **left**, **right** und **obj** die wiederum eine der 6 Variants sein können. Die rekursion endet also bei der Wahl einer der 3 Variants **Eps**, **Phi** und **Char** die keine weitere Schachtelung zulassen.

Die Zeichen des Regulären Ausdrucks werden im Value Argument **val** der Variant **Char** gespeichert. Hier kann ein beliebiger Character gewählt werden.

## **Beispiele**
Den Ausdruck __(a | b)__ bildet man so:
```
let re1 = Box::new(Exp::Alt{
    left: Box::new(Exp::Char{val : 'a'}),
    right: Box::new(Exp::Char{val : 'b'}) 
});
``` 
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
println!("This is the simplified Version of re1: {}", simplify(&re1));
```
Dieser Code hätte die Ausgabe __This is the simplified Version of re1: Conc { left: Char { val: 'a' }, right: Char { val: 'b' } }__ zur Folge.

Der Ausdruck __((a)*) *__ lässt sich wegen Regel 4 vereinfachen zu __a *__. Im Code ist das folgendermaßen umsetzbar:
```
let re2 = Box::new(Exp::Star{
    obj: Box::new(Exp::Star{
        obj: Box::new(Exp::Char{val : 'a'})
    }) 
});
println!("This is the simplified Version of re2: {}", simplify(&re2));
```
Dieser Code hätte die Ausgabe __This is the simplified Version of re2: Star { obj: Char { val: 'a' } }__ zur Folge.

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
println!("This is the simplified Version of re3: {}", simplify(&re3));
```
Dieser Code hätte die Ausgabe __This is the simplified Version of re3: Conc { left: Star { obj: Char { val: 'a' } }, right: Char { val: 'b' } }__ zur Folge.

Da diese Schreibweise aber immernoch sehr unlesbar ist, wurde zusätzlich eine PrettyPrint methode implementiert, die den Ausdruck ordentlich lesbar zurück gibt. Um diese Funktion soll es als nächstes gehen.

# Printen von Regulären Ausdrücken

Die lesbare Darstellung von Regulären Ausdrücken wird in diesem Code mit der Funktion ```pretty(x : &Exp) -> String {} ``` umgesetzt die ebenfalls einen Ausdruck vom Typ **Exp** erhält und diesen als ein String zurück gibt. 

Im Grunde arbeitet diese Funktion ähnlich wie auch ```simplify(x : &Exp) -> Exp {}```, indem sie rekursiv durch den Ausdruck schreitet und das aktuelle element, statt es zu vereinfachen, in einen String umwandelt. Bei Elementen, die wiederum ein **left**, **right** oder **obj** enthalten, wird eine tiefere Schachtelung aufgerufen.

- Alternative``` "(".to_string() + &pretty(&left) + &"|".to_string()+ &pretty(&right) + &")".to_string();```
- Concatenation: ```"(".to_string() + &pretty(&left) + &pretty(&right) + &")".to_string();```
- Star: ```"(".to_string() + &pretty(&obj) + &"*)".to_string();```

# **Aufgabenteil 2 - Transformation von Regulären Ausdrücken in Automaten**

Dieser Teil der Ausarbeitung ist noch nicht finalisiert, da die Arbeit am Code noch andauert. Gerüchten zufolge ist der Student mental am Ende weiß nicht mehr, wie er weiter machen soll. 

# **Aufgabenteil 3 - Ausführen von Automaten**
Dieser Teil der Ausarbeitung existiert noch nicht, da die Arbeit am Code noch nicht begonnen hat.