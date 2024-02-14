# Determining the radio coverage area of a digital DRM transmitter in the Greek territory

This the work folder of a university assignment that tasked us, Ermina Trontzou and Maciej Ratkiewicz, with calculating the expected terrestial coverage of a hypothetical DRM (Digital Radio Mondiale) transmitter located in Larissa, Greece.

![resulting coverage map](https://github.com/Erek-Razou/wave-propagation/blob/main/coverage.png)
_The resulting coverage map for our parameters_


The repository contains:
- [The project's paper](https://github.com/Erek-Razou/wave-propagation/blob/main/WG05_TPRE_01.pdf).
- A program that was written in Rust and was used for the coverage calculations.
- The source code given by ITU in [recommendation ITU-R P.368-10](https://www.itu.int/rec/R-REC-P.368-10-202208-I/en) for calculating the signal's strength loss according to parameters related to the signal, the transmitter and receiver, and the ground surface over which the signal propagates.
  It was modified to fix a bug and also to allow us to use the [Millington method for mixed paths](https://www.itu.int/dms_pub/itu-r/opb/hdb/R-HDB-59-2014-PDF-E.pdf)
- The [map we used](https://github.com/Erek-Razou/wave-propagation/blob/main/lines.xcf) to measure the distances of terrain changes from the transmitter.
- The [spreadsheet containing our measurements](https://github.com/Erek-Razou/wave-propagation/blob/main/data.xlsx).
- The [resulting coverage map](https://github.com/Erek-Razou/wave-propagation/blob/main/coverage.png).
- Various other miscellanious files.

## Parameters

- For the transmitter and receiver:
  - SVM (Short Vertical Monopole)
  - Power: 10 KW
  - Height: 10 m (transmitter and receiver)
- For the signal:
  - Carrier frequency 1 MHz
  - Bandwidth 9 kHz
  - Robustness mode A2
  - 64QAM, protection level 3
  - Ground wave propagation
  - Minimum usable field strength 43.2 dB(μV)/m
- For propagation:
  - Electrical parameters used for land σ=3m/s and ε=22
  - Electrical parameters used for sea σ=5m/s and ε=70


## Methodology

Our methodology in detail and a lot more can be read (in Greek) in [our paper](https://github.com/Erek-Razou/wave-propagation/blob/main/WG05_TPRE_01.pdf).
But, in short:
- We took measurements on the map in 5 degree increments in a circle surrounding our transmitter, noting down the distance at which the terrain changes from land to sea or the opposite.
- The resulting spreadsheet is converted to a plain text CSV format to be read by our program.
- Our program is equiped the project's parameters, [ITU's calculator for signal energy loss over a smooth terain](https://github.com/Erek-Razou/wave-propagation/blob/main/LFMF/include/LFMF.h) fixed and modified for our needs, and Millington's method for mixed paths.
  With all that, it can search the terrain according to the measurements looking for the closest point to the transmitter where the signal has the minimum usable field strength (dB(uV)/m), which is a parameter entered to the program when executing it.
  We define that as the transmitter's coverage for that specific direction.
- With the output date of our program, we can go back to our map, mark the points where the signal is at it's minimum usable field strength, connect the dots, and enjoy our coverage map.


## The program

To run the program, one needs to have the Rust compiler and toolchain installed and then it's just a matter of running `cargo run --release -- 43.2 data.csv`.
43.2 can be replaced with any other minimum field strength value is desirable.

A big bottleneck for the program's execution is the graphs the program plots for each angle it processes.
Plotting can be disabled with the appropriate flag, like so: `cargo run --release -- 43.2 data.csv --no-plot`.

The search algorithm is basically a linear search but with 2 stages and inverse step scaling.
It has decent performance and, most importantly, resilience against the over-sea recovery effect we were seeing, which caused the signal to pick up in strength whenever the terrain turned to sea.
This effect threw off our inital attempts to use faster approaches like the bisection method/binary search.

The algorithm has gone through several tests to check its robustness and those can be run with `cargo test --release`.


## Thank you 

We want to thank [Anastasios D Papatsoris](https://www.researchgate.net/profile/Anastasios-Papatsoris) for his teachings, guidance, and this fun and insightful assignment.

