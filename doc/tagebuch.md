# Softwareprojekttagebuch

| Datum      | Tätigkeit                                                                                                                                                                                |              Dauer |
|------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------:|
| 13.04.2023 | Erstes Teammeeting                                                                                                                                                                       |               1,5h |
| 14.04.2023 | Socket.io für Rust angesehen                                                                                                                                                             |                 1h |
| 15.04.2023 | Testprojekt in Rust                                                                                                                                                                      |                 1h |
| 18.04.2023 | Anfang Erstellung technische Spezifikation                                                                                                                                               |                 1h |
| 19.04.2023 | Vorstellung und Projektmeeting                                                                                                                                                           |               2,5h |
| 20.04.2023 | Projektmeeting                                                                                                                                                                           |              45min |
| 27.04.2023 | Projektmeeting                                                                                                                                                                           |             40 min |
| 27.04.2023 | Rust Basis erstellt                                                                                                                                                                      |                 1h |
| 02.05.2023 | Basis für Authentikation erstellt                                                                                                                                                        |                 2h |
| 03.05.2023 | Verbundene Spieler abfragen und Basis für SocketIO<br/>Recherche für SocketIO in Rust<br/>Schwierigkeiten mit der Dokumentation von rust_socketio                                        |    2h<br/>6h<br/>- |
| 04.05.2023 | Projektmeeting                                                                                                                                                                           |                 1h |
| 09.05.2023 | Spiele können erstellt werden                                                                                                                                                            |                 3h |
| 10.05.2023 | JSON-Objekte als Rust-structs angelegt um autom. parsen zu ermöglichen<br/>Code cleanup<br/>Kommentare hinzugefügt                                                                       |                 3h | 
| 11.05.2023 | Projektmeeting                                                                                                                                                                           |              45min |
| 11.05.2023 | Logik zum Spieleeinladungs-Callback hinzugefügt - Spiele werden automatisch angenommen                                                                                                   |                 1h |
| 11.05.2023 | README angepasst                                                                                                                                                                         |              30min |
| 18.05.2023 | Konsolenmenü angedacht + Code cleanup                                                                                                                                                    |                 1h |
| 18.05.2023 | Projektmeeting                                                                                                                                                                           |              45min |
| 23.05.2023 | Fehler im Menü ausgebessert + Menü aufgebaut                                                                                                                                             |                 3h |
| 24.05.2023 | Menü erweitert + erste simple Spiellogik                                                                                                                                                 |                 3h |
| 25.05.2023 | Turniereinladungs-Callback eingepflegt                                                                                                                                                   |              10min |
| 25.05.2023 | Projektmeeting und Probeturnier                                                                                                                                                          |              90min |
| 27.05.2023 | env-variablen für 2. Client konfiguriert + Menüoptionen für starten mit 2. Client hinzugefügt<br/> Cleanup                                                                               | -<br/>2h<br/>30min |
| 30.05.2023 | neue JSON-Objekt-Änderungen des Server-Teams implementiert + Cleanup                                                                                                                     |              30min |
| 30.05.2023 | Codereview für PR [#1](https://github.com/Nope-Cardgame/Rust-Client/pull/1) und [#2](https://github.com/Nope-Cardgame/Rust-Client/pull/4) mit [Jan Rasche](https://github.com/Muquinbla) |                 2h |
| 01.06.2023 | Logik für Aktionskarten hinzugefügt                                                                                                                                                      |                 3h |
| 01.06.2023 | Projektmeeting                                                                                                                                                                           |                 1h |
| 05.06.2023 | Fehler der Logik für Aktionskarten gefixt                                                                                                                                                |                 2h |
| 09.06.2023 | Weiterführende Logik eingebaut - mögliche Karten werden herausgefiltert und dann absteigend nach Farbanzahl sortiert -> höhere Chance nope!-Runden zu spielen                            |                 2h |
| 09.06.2023 | Am Nachmittag Testturniere für alle Gruppen organisiert um letzte Fehler auszumerzen                                                                                                     |                 4h |
| 09.06.2023 | Turnier                                                                                                                                                                                  |              90min |
|            |                                                                                                                                                                                          |                    |
| 13.06.2023 | Dokumentation - anlegen technischer Dokumentation                                                                                                                                        |              30min |
|            |                                                                                                                                                                                          |           57h05min |


Insgesamt ist der Projektdurchlauf sehr gut gelungen.
Anfangs hatte ich mir etwas Sorgen gemacht den Client alleine aufzubauen.

Im Endeffekt konnte ich dadurch aber gute Erfahrungen sammeln, dass ich mich um jeglichen Aspekt alleine kümmern musste.

Lediglich den Umfang der Einarbeitung in unbekanntes Rust-Territorium und das socketIO crate für Rust, habe ich etwas unterschätzt.
Das crate war leider etwas dürftig (im Gegensatz zu anderen SocketIO plugins) dokumentiert, wodurch der Aufbau der Verbindungen und Callbacks sehr viel Trial-And-Error und Zeit gebraucht hat.

Es war auch manchmal etwas schwierig, die Zeit richtig für das Projekt und die anderen Module aufzuteilen.

Ein großer Benefit aus diesem Projekt ist, dass ich mich aus der Komfortzone der bekannten Sprachen
gewagt habe und nun die Basics einer neuen, beliebten Sprache beherrsche.

---

Aus der Sicht des Projektmanagements ist dieses Projekt sehr gut abgelaufen. Kevin hat seine Aufgaben als Projektleiter
super bewältigt.

Kommunikation mit, und Hilfsbereitschaft von anderen Teams war hervorragend.

Letztlich hat das Serverteam herausragende Leistung erbracht. Viele Elemente der Serverstruktur wurden
proaktiv implementiert und falls man Probleme mit der Client-Server-Kommunikation hatte, konnte
man jederzeit Hilfe anfragen. Selbst noch 11 Uhr nachts!