/*
    State utils for unittests and integration test
    After cuple hours of thinking, I decided create
    module with state for unit or integration tests.

    Main reason why I create this, is DRY and flow test control.

    What is flow test control? Flow test control - this is extend test without
    it's override or double create functions.
    What is this mean? This is when you can create scenario like setups for different tests.
    Main reason of this is easy extendable code structure with strong ordered flow.

    How I am going to implement this?? Mmm, realy, my best thoughts is create singleton like structure.
    `static` type in rust with extendable structure with builder.

    Main conception is - make helper for easy maintain integration and unit tests.

    Structure? This is good question.
    I think, first of all, need to create `StateHelper`.

    StateHelper:
        call_stack() -> method hold call stack functions (will call functions in order of add)
        truncate_table() -> Method should truncate choosen table after tests
        add_constant() -> Method should add constant witj

        // But need think about scenario implementation
        add_scenario() -> Method should add scenario for run in tests with predefined constants and call_stack.
    
    I think this should be in different branch, with this interesting feature.
    This is not all methods, that need add to the StateHelper. I missed some methods. But I will decribe them.


*/