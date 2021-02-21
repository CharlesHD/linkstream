#!/usr/bin/env bash

OLDDIR=`pwd`
OUTDIR="../target/tests/"$1"/"
cd script/

PDIR=$OUTDIR"plots/"
mkdir -p $PDIR
PSCRIPT=$1".plot"
cp $PSCRIPT $PDIR
cd $PDIR
gnuplot $PSCRIPT
rm $PSCRIPT
cd $OLDDIR
