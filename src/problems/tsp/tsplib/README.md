# TSPLIB

## Symmetric traveling salesman problems.

These are from the [Heidelberg University](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/).

### Modifications

Each `opt.tour` file has been extended with a `BEST_SOLUTION` attribute. The best solution for each problem in TSPLIB can be
found [here](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/STSP.html). 
The lower bound was used instead if no best solution is currently known.

The parser currently only supports 2D weights with `EUC_2D`, `MAN_2D` or `MAX_2D` weights.
