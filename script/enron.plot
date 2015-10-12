set terminal pdf enhanced

if (system("[ ! -e ../existence.matrix ]; echo $?")) {
    set output "exist_mat.pdf"
    set xlabel "time (s)"
    set ylabel "node"
    plot "../existence.matrix" matrix with image title "enron : matrice d'existence"
}

set grid ytics lt 0 lw 1 lc rgb "#bbbbbb"
set grid xtics lt 0 lw 1 lc rgb "#bbbbbb"
set xlabel "delta (days)"

if (system("[ ! -e ../cut.dat ]; echo $?")) {
    set ylabel "boxes number"
    set output "cut.pdf"
    plot "../cut.dat" using ($1/86400):2 notitle
}

if (system("[ ! -e ../lr.dat ]; echo $?")) {
    set ylabel "largest rectangle area"
    set output "lr.pdf"
    plot "../lr.dat" using ($1/86400):($4/(98277034*150)) notitle w lines
}

if (system("[ ! -e ../box_moy.dat ]; echo $?")) {
    set ylabel "average box area"
    set output "box_moy.pdf"
    plot "../box_moy.dat" using ($1/86400):2 notitle
}

if (system("[ ! -e ../comps_low.dat ]; echo $?")) {
    set xlabel "delta (days)"
    set ylabel "# nodes" 
    set output "nb_comps.pdf"
    plot  "../comps_low.dat" using ($1/84600):3 notitle w lines, "../comps_up.dat" using ($1/84600):3 notitle w lines lt rgb "blue"
}

unset grid
if (system("[ ! -e ../part_low.dat ]; echo $?")) {
    set xlabel "time (days)"
    set ylabel "# nodes"
    set output "part.pdf"
    plot "../part_up.dat" using ($1/86400):4 notitle w boxes fs solid 0.7, "../part_low.dat" using ($1/86400):4 notitle w boxes fs solid 0.7 lt rgb "blue", 150 lt rgb "black" notitle
}
