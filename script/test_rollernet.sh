#!/bin/bash

CMD="../target/release/linkstream"
DATA="../datas/rollernet.dyn.gz "
OUTDIR="../target/tests/rollernet/"
NBNODES=62
DELTA=300
OLDDIR=`pwd`

cd script
mkdir -p $OUTDIR

### Existence Matrix
zcat $DATA | $CMD calc exist $DELTA $NBNODES > $OUTDIR"existence.matrix"

### Partitions
zcat $DATA | $CMD calc part $DELTA $NBNODES > $OUTDIR"part_low.dat"
zcat $DATA | $CMD calc part up $DELTA $NBNODES > $OUTDIR"part_up.dat"

# ### Components
# OUT=$OUTDIR"comps_low.dat"
# OUT2=$OUTDIR"comps_up.dat"
# rm $OUT $OUT2
# for ((i=50; i <= 360; i++)) do
# (
#     ((number=i*5))
#     echo "components for delta = "$number
#     echo $number `zcat $DATA | $CMD calc comps $number $NBNODES` >> $OUT
#     echo $number `zcat $DATA | $CMD calc comps up $number $NBNODES` >> $OUT2
# )
# done

# ### Number of existence intervals
# OUT=$OUTDIR"cut.dat"
# rm $OUT
# for ((i=1; i <= 200; i++)) do
# (
#     ((number=i))
#     echo "number of existence intervs for delta = "$number
#     echo $number `zcat $DATA | $CMD calc exist cut $number $NBNODES | wc -l` >> $OUT
# )
# done

# ### Largest rectangle
# OUT=$OUTDIR"lr.dat"
# rm $OUT
# for ((i=1; i <= 160; i++)) do
# (
#     ((number=i*3))
#     echo "largest rectangle : "$number
#     echo $number `zcat $DATA | $CMD calc exist lr $number $NBNODES` >> $OUT
# )
# done

# ### Boxes medium size
# OUT=$OUTDIR"box_moy.dat"
# rm box_moy.dat
# for ((i=1; i <= 400; i++)) do
# (
#     ((number=i*10))
#     echo "medium boxe size for delta = "$number
#     echo $number `zcat $DATA | $CMD calc exist cut $number $NBNODES | awk '{n1+=($2-$1)*($NF - 2); n2+=1} END { print n1/n2/(62*9976)}' `>> $OUT
# )
# done

    cd $OLDDIR
