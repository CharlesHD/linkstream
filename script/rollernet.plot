set terminal pdf enhanced

if (system("[ ! -e ../existence.matrix ]; echo $?")) {
    set output "exist_mat.pdf"
    set xlabel "temps (s)"
    set ylabel "sommet"
    plot "../existence.matrix" matrix with image title "rollernet : matrice d'existence";
    unset xlabel
    unset ylabel
}

set grid ytics lt 0 lw 1 lc rgb "#bbbbbb"
set grid xtics lt 0 lw 1 lc rgb "#bbbbbb"

if (system("[ ! -e ../cut.dat ]; echo $?")) {
    set output "cut.pdf"
    plot "../cut.dat" using 1:2 notitle
}

if (system("[ ! -e ../lr.dat ]; echo $?")) {
    set output "lr.pdf"
    plot "../lr.dat" using 1:($4/(9976*62)) notitle w lines
}

if (system("[ ! -e ../box_moy.dat ]; echo $?")) {
    set output "box_moy.pdf"
    plot "../box_moy.dat" using 1:2 notitle w lines
}

if (system("[ ! -e ../comps_low.dat ]; echo $?")) {
    set xlabel "Delta (s)"
    set ylabel "# sommets$
    set output "nb_comps.pdf"
    plot  "../comps_low.dat" using 1:3 notitle w lines, "../comps_up.dat" using 1:3 notitle w lines lt rgb "blue"
}

unset grid
if (system("[ ! -e ../part_low.dat ]; echo $?")) {
    set xlabel "temps (s)"
    set ylabel "# sommets"
    set output "part.pdf"
    plot "../part_up.dat" using 1:4 notitle w boxes fs solid 0.7, "../part_low.dat" using 1:4 notitle w boxes fs solid 0.7 lt rgb "blue", 62 lt rgb "black" notitle
}