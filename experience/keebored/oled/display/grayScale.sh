#!/bin/bash
for img in frame_*.png; do
  convert "$img" -colorspace Gray -threshold 50% bw_"$img"
done
