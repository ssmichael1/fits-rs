#!/usr/bin/env bash

# Download FITS files used to test this library
# These example files are provided by NASA

testdir=testfiles
if [ ! -d $testdir ]; then
    mkdir -p $testdir
fi

## List of files
declare -a files=(
    "https://fits.gsfc.nasa.gov/samples/WFPC2u5780205r_c0fx.fits"
    "https://fits.gsfc.nasa.gov/samples/WFPC2ASSNu5780205bx.fits"
    "https://fits.gsfc.nasa.gov/samples/FOCx38i0101t_c0f.fits"
    "https://fits.gsfc.nasa.gov/samples/FOSy19g0309t_c2f.fits"
    "https://fits.gsfc.nasa.gov/samples/HRSz0yd020fm_c2f.fits"
    "https://fits.gsfc.nasa.gov/samples/NICMOSn4hk12010_mos.fits"
    "https://fits.gsfc.nasa.gov/samples/FGSf64y0106m_a1f.fits"
    "https://fits.gsfc.nasa.gov/samples/UITfuv2582gc.fits"
    "https://fits.gsfc.nasa.gov/samples/IUElwp25637mxlo.fits"
    "https://fits.gsfc.nasa.gov/samples/EUVEngc4151imgx.fits"        
)

## now loop through the above array
for i in "${files[@]}"
do   
   filename=$testdir/$(basename $i)
   if [ ! -f $filename ]; then
        echo "Downloading" $filename
        curl -o $filename $i
   fi
done
