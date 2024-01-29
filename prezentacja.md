# Strona tytułowa
Dzień dobry,
Nazywam się Łukasz Niedzielski
Przygotowałem pracę pod tytułem:
 Implementacja interpretera języka programowania Lox

# Cel i zakres pracy
Celem pracy była implementacja interpretera, czyli programu pozwalającego na ewaluację programów napisanych zgodnie z regułami języka Lox.

Praca także opisuje elementy i konstrukcje języka Lox, za co one odpowiadają, jakie są interakcje między nimi.

# Główne zagadnienia

W pracy omówiono pojęcia związane z językami programowania: paradygmat strukturalny, jakie cele spełniają konstrukcje językowe.
w pracy omówiono także techniki budowy programów przetwarzających języki, mam tu na myśli translacja - proces tłumaczenia języka na inną formę lub proces interpretacji programu znajdującego się w formie gotowej do wykonania.
Język Lox implementuje paradygmat programowania imperatywnego, czyli instrukcje są wykonywane jedna po drugiej, manipulacja stanu programu występuje poprzez instrukcje,  oraz strukturalnego czyli język wspiera pojęcia takie jak procecedury (tutaj nazwane funkcjami), bloki kodu, instrukcje warunkowe.
Omawiana jest także statyczna analiza kodu, czyli analiza struktury drzewa składniowego, czyli kodu źródłowego przetłumaczonego na reprezentację gotową to wykonania.

# Inne prace

Podobne programy są opisane w książce Crafting Interpreters autorstwa Roberta Nystorm, istnieje wiele implementacji interpreterów oraz kompilatorów języka Lox na githubie.

Projekty działające na podobnym poziome, czyli wykonują program napisany w pewnym języku programowania, to nodejs - środowisko uruchomieniowe dla języka JavaScript lub cpython - referencyjna implementacja interpretera języka python.

# Technologie

Implementacja interpretera została przygotowana z wykorzystaniem języka rust oraz zestawu narzędzi cargo w celu budowy projektu oraz testowania programu. Język rust cechuje się bardzo rozbudowanym systemem typów który ułatwia tworzenie abstrakcji oraz wymusza obsługę wszystkich przypadków w jakich mogą znajdować obiekty zarządzane przez interpreter. Język rust ma wiele zaporzyczeń z języków funkcyjnych, najczęściej używanym zaporzyczeniem jest pattern matching czyli deklaratywny sposób sprawdzania czy obiekty mają oczekiwaną strukturę. 

# Język programowania

Język programowa to zbiór zasad budowy konstrukcji (czyli gramatyka) oraz opis znaczenia poszczególnych konstruki (semantyka).

# Interpreter
Program komputerowy implementujący reguły składnni oraz obługę konstrukcji zgodnine z opisem wybranyego języka.
Wykorzystuje procesy analizy leksykalnej oraz składniowej aby sprawdzić poprawność struktury programu oraz utworzyć jego wewnętrznną reprezentację - AST

# Analiza składniowa oraz leksykalna

AST - abstract syntax tree - drzewiasta struktura danych, będąca listą instrukcji zbudowanych ze słów kluczowych, identyfikatorów, wyrażeń oraz bloków instrukcji.

Rozpoznaje błędy składniowe, takie jak nieoczekiwany token, nieznany lub nieoczekiwany symbol.

# Uruchomienie programu

Wykonanie każdej instrukcji znajdującej się w zakresie globalnym w kolejności od góry do dołu. 

# Stan programu

Mechanizm służący zapamiętaniu rezultatów zmian w stanie programu, 

# Analiza statyczna



