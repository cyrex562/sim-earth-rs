Starting SimEarth
-----------------

To start SimEarth:

1.      Run the install program to decompress the program file, set the
default directory, video and sound options.

2.      Change to the drive that contains the directory where you installed
SimEarth.

3.      Change to the directory where you installed SimEarth.

4.      Type Simearth and press the Enter key.

The program will then load and display the SimEarth title screen.  Press
the space bar or mouse button to clear the title screen and you will be
presented with a tutorial window.

If your machine is capable of displaying more than one of the supported
video modes, SimEarth will allow you to switch to a different mode from
the command line.  You will first have to either run the install program
to copy the appropriate files for each video mode or you can copy all the
files from all the data disks to your SimEarth directory.  Remember if you
run the install program, the default settings will be set to whatever was
selected the last time you ran the install program.

Once the you have installed all the video modes, you may use the following
command line switches to override the default options:

Switch  Graphic mode
/dV     Sets VGA 16 colors 640x480
/dE     Sets EGA 16 colors 640x480
/de     Sets EGA 16 colors 320x200
/d2     Sets MCGA 256 colors 320x200
/dT     Sets Tandy 16 colors 320x200
/dM     Sets EGA mono 640x350
/dm     Sets VGA mono 640x480
/dH     Sets Hercules 720x348

Switch  Sound board
/sa     Adlib
/sb     Soundblaster
/sc     Covox
/si     IBM internal
/st     Tandy

For example, to run SimEarth in the EGA lores mode with Adlib sound type:

SIMEARTH /de /sa

and press the Enter key.


Ghost Mouse
-----------

If you run SimEarth using Microsoft's mouse driver version 7.00 in VGA or
EGA Hires modes, Ghost cursor images may appear on your screen.  To correct
this problem you may either run SimEarth using the /B command line option
or order the latest mouse driver from Microsoft.  The /B command line option
may slow down the simulation speed.


Querying in the compressed edit window
-------------------------------------

Because the compressed edit window only displays every other tile, when
you do a query on a tile it may not display all the information.  If you
would like to see all the information, turn off the compress edit window
option.


Amstrad VGA/EGA cards
---------------------

The VGA/EGA display mode on Amstrad PC computers does not support the
640x350 or 640x480 display size.  Please select the 320x200 when using the
install program.

_________________________________________________________________________

Lancement de SimEarth
---------------------

Pour lancer SimEarth:

1.       ExÇcutez le programme Installe pour dÇcomprimer le fichier
programme, sÇlectionnez le rÇpertoire systÇmatique (par dÇfaut), les
options vidÇo et son.

2.       Passez Ö l'unitÇ qui contient le rÇpertoire oó vous avez installÇ
SimEarth.

3.       Passez au rÇpertoire oó vous avez installÇ SimEarth.

4.       Tapez Simearth et pressez la touche Enter.

Le programme se chargera alors et le titre SimEarth s'affichera Ö l'Çcran.
Pressez la barre d'espace ou le bouton de la souris et une fenàtre
d'initiation vous sera prÇsentÇe.

Si votre machine est capable d'afficher plus qu'un des modes vidÇo pris en
charge, SimEarth vous permettra de passer Ö un diffÇrent mode Ö partir de la
ligne de commande. Pour cela vous devrez d'abord soit exÇcuter le programme
Installe, pour copier les fichiers adÇquats pour chaque mode vidÇo, soit
copier tous les fichiers de tous les disques de donnÇes sur votre rÇpertoire
SimEarth.
N'oubliez pas que si vous exÇcutez le programme Installe, les paramätres
systÇmatiques seront ceux qui ont ÇtÇ sÇlectionnÇs la derniäre fois que vous
avez exÇcutÇ le programme Installe.

Däs que vous avez installÇ tous les modes vidÇo, vous pouvez vous servir des
options suivantes de la ligne de commande pour annuler les options
systÇmatiques:

Option            Mode graphique
/dV               SÇlectionne VGA 16 couleurs 640x480
/dE               SÇlectionne EGA 16 couleurs 640x480
/de               SÇlectionne EGA 16 couleurs 320x200
/d2               SÇlectionne MCGA 256 couleurs 320x200
/dM               SÇlectionne Tandy 16 couleurs 320x200
/dM               SÇlectionne EGA mono 640x350
/dm               SÇlectionne VGA mono 640x480
/dH               SÇlectionne Hercules 720x348

Option            Carte Son
/sa               Adlib
/sb               Soundblaster
/sc               Covox
/si               IBM interne
/st               Tandy

Par exemple, pour lancer SimEarth au mode EGA Faible RÇsolution avec Adlib
comme type de son, tapez:

SIMEARTH /de /sa

et pressez la touche Enter.

Double Souris
-------------

Si vous lancez SimEarth en vous servant d'un logiciel pilote de souris
Microsoft version 7.00 aux modes VGA ou EGA Haute RÇsolution, il se peut que
des images secondaires du curseur apparaissent sur votre Çcran. Pour rectifier
ce probläme, vous pouver soit lancer SimEarth en vous servant de l'option /B
de la ligne de commande, soit commander le dernier logiciel pilote de souris
de Microsoft. L'option /B de la ligne de commande peut ralentir la vitesse de
simulation.

Consultation de la fenàtre Çdite condensÇe
------------------------------------------

La fenàtre Çdite condensÇe n'affichant qu'une partie des titres, lorsque vous
la consultez sur un titre, il se peut qu'elle n'affiche pas toute
l'information. Si vous dÇsirez voir l'information dans son intÇgralitÇ,
neutralisez l'option fenàtre Çdite condensÇe.

Cartes Amstrad VGA/EGA
----------------------

Le mode d'affichage VGA/EGA sur les PC Amstrad ne prend pas en charge
l'affichage de dimension 640x350 ou 640x480. S.V.P sÇlectionnez 320x200
lorsque vous utilisez le programme Installe.

___________________________________________________________________________

***** Le "CONFIG.SYS" doit inclure les lignes
                            FILES = 10
                            BUFFERS = 10
___________________________________________________________________________

SimEarth starten
----------------

Zum Starten von SimEarth gehen Sie bitte wie folgt vor:

1. Rufen Sie zuerst das Installationsprogramm 'INSTALL.EXE' auf. Es
dekomprimiert die Programmdateien, erstellt das Unterverzeichnis und stellt das
Programm auf Ihre Grafikkarte und die Sound-Karte ein.

2. Schalten Sie nun auf das Laufwerk um, auf das SimEarth kopiert wurde.

3. Danach schalten Sie in das Unterverzeichnis um, in das SimEarth installiert
wurde.

4. Tippen Sie nun bitte 'SIMEARTH', und bestÑtigen Sie den Befehl mit RETURN
oder ENTER.

Das Programm wird jetzt geladen und prÑsentiert Ihnen als erstes den
Titelbildschirm. Mit der Leertaste oder einer Maustaste gelangen Sie danach zum
Tutorial-Bildschirm.

Falls Ihre Grafikkarte mehr als eine der folgenden Grafikmodi beherrscht,
kînnen Sie bereits beim Aufrufen von SimEarth auf der Kommandozeile auf einen
anderen Grafikmodus umschalten. Allerdings mÅssen Sie hierfÅr vorher die
notwendigen Dateien entwe der mit Hilfe des Installationsprogrammes
installieren oder einfach von den Datendisketten in den SimEarth-Ordner
kopieren. Hinweis: Beim erneuten Aufruf des Installationsprogrammes werden die
Einstellungen des letzten Aufrufes verwendet.

Sobald Sie alle Grafikmodi installiert haben, kînnen Sie mit folgenden Schalter
beim Programmstart die Standardeinstellung au·er Kraft setzen:

Schalter  Grafikmodus
/dV       VGA mit 16 Farben 640x480
/dE       EGA mit 16 Farben 640x480
/de       EGA mit 16 Farben 320x200
/d2       MCGA mit 256 Farben 320x200
/dT       Tandy mit 16 Farben 320x200
/dM       EGA Monochrom 640x350
/dm       VGA Monochrom 640x480
/dH       Hercules 720x348

Schalter  Sound-Karte
/sa       Adlib
/sb       Soundblaster
/sc       Covox
/si       IBM internal
/st       Tandy

Um SimEarth zum Beispiel mit den Einstellungen 'EGA niedrige Auflîsung' und
'ADLIB'-Karte zu starten, geben Sie zum Programmstart folgenden Befehl ein:

SIMEARTH /de /sa

und bestÑtigen mit ENTER oder RETURN.

Mausgeister
-----------

Falls Sie SimEarth zusammen mit dem Microsoft Maustreiber Version 7.0 in VGA-
oder EGA-Modus starten, erscheinen ab und zu wie von Geisterhand Abbildungen
des Mauszeigers auf dem Bildschirm. Dieses Problem kînnen Sie entweder mit
einer aktuelleren Ve rsion des Maustreibers lîsen oder mit Hilfe des Schalters
'/B' beim Programmstart umgehen. Die Option '/B' verlangsamt das Tempo der
Simulation.

Abfragen im kleinen Arbeitsfenster
----------------------------------

Da im kleinen Arbeitsfenster nur bestimmte Informationen eingeblendet werden,
erhalten Sie bei Abfragen eventuell nicht alle gewÅnschten Details angezeigt.
Falls Sie alle verfÅgbaren Informationen benîtigen, schalten Sie bitte die
Option 'compress ed it window' ab.


Amstrad VGA/EGA Karten
----------------------

Die Option VGA/EGA von SimEarth unterstÅtzt auf Amstrad PCs nicht die 640x350
oder 640x480 Auflîsung. WÑhlen Sie bitte anstatt dessen im
Installationsprogramm die Auflîsung 320x200.
______________________________________________________________________________

****  FÅgen Sie foigende Zelle in die "CONFIG.SYS" Datel ein:
                            FILES = 10
                            BUFFERS = 10
------------------------------------------------------------------------------
