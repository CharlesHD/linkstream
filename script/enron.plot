set terminal pdf enhanced monochrome dashed

s2w(x)= x / (86400 * 7)

if (system("[ ! -e ../existence.matrix ]; echo $?")) {
    set output "exist_mat.pdf"
    set xlabel "time (s)"
    set ylabel "node"
    plot "../existence.matrix" matrix with image title "enron : matrice d'existence"
}

set grid ytics lt 0 lw 1 lc rgb "#bbbbbb"
set grid xtics lt 0 lw 1 lc rgb "#bbbbbb"
set xlabel "delta (weeks)"

if (system("[ ! -e ../cut.dat ]; echo $?")) {
    set ylabel "boxes number"
    set output "cut.pdf"
    plot "../cut.dat" using ($1/604800):2 notitle
}

if (system("[ ! -e ../lr.dat ]; echo $?")) {
    set ylabel "largest rectangle area"
    set output "largest_rectangle.pdf"
    plot "../lr.dat" using (s2w($1)):($4/(98277034*150)) notitle w lines
}

if (system("[ ! -e ../box_moy.dat ]; echo $?")) {
    set ylabel "average box area"
    set output "box_moy.pdf"
    plot "../box_moy.dat" using (s2w($1)):2 notitle
}

if (system("[ ! -e ../comps_low.dat ]; echo $?")) {
    set xlabel "delta (weeks)"
    set ylabel "# nodes"
    set output "nb_comps.pdf"
    plot  "../comps_low.dat" using (s2w($1)):3 notitle w lines, "../comps_up.dat" using (s2w($1)):3 notitle w lines lt rgb "blue"
}

unset grid
if (system("[ ! -e ../part_low.dat ]; echo $?")) {
    set xlabel "time (weeks)"
    set ylabel "# vertex (%)"
    set output "part.pdf"
plot "../part_up.dat" using (s2w(($1 + $2)/2)):($4*100/150):(s2w($1)):(s2w($2)) notitle w xerrorbars ps 0 lt 1 lc rgb "#aaaaff", "../part_low.dat" using (s2w(($1 + $2)/2)):($4*100/150):(s2w($1)):(s2w($2)) notitle w xerrorbars ps 0 lt 1 lc rgb "#FFBBBB", "../part_low.dat" using (s2w(($1 + $2)/2)):($4*100/150) notitle w lines lt 1 lw 2 lc rgb "#0000FF", "../part_up.dat" using (s2w(($1 + $2)/2)):($4*100/150) notitle w lines lt 1 lw 2 lc rgb "#E63500"
}
