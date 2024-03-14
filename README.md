# The chaos_dna utility for generating randomly mutated DNA sequences

## Probability modifications
The ins_prob, del_prob, and mod_prob variables (defined near the top of main.rs) control the probabilities of an insertion, a deletion, and/or a mutation error occuring at each index.  The default probabilities are set to 0.2, 0.2, and 0.3 respectfully; however, these can be modified for any value between 0 and 1.

## Output style
An example output for the input "ACGTGCA" is the following:

--- Modified sequence ---
GTTGGGAG
--- Changelog ---
AC GT GC A
<span style="color:rgb(56,97,140)">G</span>-T-TGG-GAG

Under modified sequence, the randomly mutated sequence is outputted; under changelog, the original sequence and randomly mutated sequence are aligned (original sequence on top, mutated on bottom). The mutated sequence is presented here with colour-coded formatting to display how the original sequence has been modified. At points where an insertion occured, a space is added to the original sequence to preserve alignment, and at points where a deletion occured, a "-" is added to the modified sequence. 

In the modified sequence:
1. Blue bases represent a mutation (base swap) error
2. Yellow bases represent an insertion
3. Red dashes (-) represent a deletion