set terminal pdf enhanced monochrome

if (system("[ ! -e ../existence.matrix ]; echo $?")) {
    set output "exist_mat.pdf"
    set xlabel "time (s)"
    set ylabel "node"
    plot "../existence.matrix" matrix with image title "rollernet : matrice d'existence";
    unset xlabel
    unset ylabel
}

set grid ytics lt 0 lw 1 lc rgb "#bbbbbb"
set grid xtics lt 0 lw 1 lc rgb "#bbbbbb"
set xlabel "delta (s)"

if (system("[ ! -e ../cut.dat ]; echo $?")) {
    set ylabel "boxes number"
    set output "cut.pdf"
    plot "../cut.dat" using 1:2 notitle
}

if (system("[ ! -e ../lr.dat ]; echo $?")) {
    set ylabel "largest rectangle area"
    set output "lr.pdf"
    plot "../lr.dat" using 1:($4/(9976*62)) notitle w lines
}

if (system("[ ! -e ../box_moy.dat ]; echo $?")) {
    set ylabel "average box area"
    set output "box_moy.pdf"
    plot "../box_moy.dat" using 1:2 notitle w lines
}

if (system("[ ! -e ../comps_low.dat ]; echo $?")) {
    set xlabel "delta (s)"
    set ylabel "# nodes"
    set output "nb_comps.pdf"
    plot  "../comps_low.dat" using 1:3 notitle w lines, "../comps_up.dat" using 1:3 notitle w lines lt rgb "blue"
}

unset grid
if (system("[ ! -e ../part_low.dat ]; echo $?")) {
    set xlabel "time (min)"
    set ylabel "# vertex (%)"
    set output "part.pdf"
    set yrange [0:100]
    plot "../part_up.dat" using (($1 + $2)/(2 * 60)):($4 * 100 / 62):($1/60):($2/60) notitle w xerrorbars ps 0 lw 1 lc rgb "#aaaaff", "../part_low.dat" using (($1 + $2)/(2*60)):($4 * 100 / 62):($1/60):($2/60) notitle w xerrorbars ps 0 lt 1 lc rgb "#FFBBBB", "../part_up.dat" using (($1 + $2)/(2*60)):($4 * 100 / 62) notitle w lines lt 2 lw 2 linecolor rgb "#0000ff", "../part_low.dat" using (($1 + $2)/(2*60)):($4 * 100 / 62) notitle w lines lt 0 lw 5 lc rgb "#E63500"
    # set output "part_gray.pdf"
    # plot "../part_up.dat" using (($1 + $2)/(2 * 60)):($4 * 100 / 62):($1/60):($2/60) notitle w xerrorbars ps 0 lw 1 lc rgb "#8888ff", "../part_low.dat" using (($1 + $2)/(2*60)):($4 * 100 / 62):($1/60):($2/60) notitle w xerrorbars ps 0 lt 1 lc rgb "#DAE074", "../part_up.dat" using (($1 + $2)/(2*60)):($4 * 100 / 62) notitle w lines lt 2 lw 2 linecolor rgb "#1D1D1D", "../part_low.dat" using (($1 + $2)/(2*60)):($4 * 100 / 62) notitle w lines lt 0 lw 5 lc rgb "#646464"
}
