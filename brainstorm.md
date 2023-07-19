# Possibilities
* make Dataflow more generic so you could be run in on asm ir
* make register aware of original location instead of ir register

Ok problem I am solving is that that I want to do some optimizations in IR but that create problem for locations of the register and instructions since I cannot use InstUUD as originally intended (aka it could be anywhere).

## First solution
* It would require to create more generic Function lattice and create new type of the live analysis
* It could help with performance since you would do this analysis on the already optimized code and it would allow for better allocation and less information to be processed

## Second solution
* This would require to add information of the source IR instruction into ASM IR instruction
* On the other hand it could prove useful to add them there anyways since it could make easier other parts of the program
