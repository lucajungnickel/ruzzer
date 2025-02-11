Own Fuzzer Implementation

generational black-box fuzzer that uses a custom grammar

Inspiration: https://www.ndss-symposium.org/wp-content/uploads/2023/02/ndss2023_f217_paper.pdf

Schiller, Nico & Chlosta, Merlin & Schloegel, Moritz & Bars, Nils & Eisenhofer, Thorsten & Scharnowski, Tobias & Domke, Felix & Schönherr, Lea & Holz, Thorsten. (2023). Drone Security and the Mysterious Case of DJI's DroneID. 10.14722/ndss.2023.24217. 

The given paper fuzzed a DJI drone on RF level and found bugs which would make it able to take control of a drone in midflight or crash it while flying.
I would like to implement a custom fuzzer, which could be applied in the same scenario: a generational black-box fuzzer which uses a custom grammar.
Lets break this down for better understanding:
- generational fuzzer: test inputs are automatically generated and then feed into the SUT ( system-under-test)
- black-box: this refers to the access to our SUT. We don't know what is happening exactly inside, but can measure / obtain results of our tested inputs
- grammar-based fuzzer: our fuzzer is generates test-input based on a defined grammar, or set of rules. This is useful if we know something about how our SUT handles the input, like the used protocol for data transmision


Our fuzzer has the following classes and data structures:

Seeder()
    seed()

    Generates a seed.

RandomSeeder()
    seed()

MutationSeeder()
    seed()
    - mutateSeed()

GrammarSeeder()
    seed()

Grammar()
    context-free grammar
    start symbol
    set of expansion rules ( also called rules)
    



Runner()
    state (
        FAIL
        PASS
        UNRESOLVED
    )    
    run()

ProgramRunner()
    run()
    runProcess()
- runs a process

PrintRunner()
    run()
- prints the seeds to stdout


Fuzzer()
    fuzz(seeds, Runner)

    Combines a Runner and Seeder.
    Generates new seeds and passes them to the runner.



