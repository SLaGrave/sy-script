# Langtest

This contains a script, which if you run will run the Syscript language test suite.

## Add a test

To add a test, simply add two files - `<testname>.sy` & `<testname>.case`. The .sy script should output something to stdout. The .case file should contain EXACTLY what that output should be. When running the tests, if the output is different at all, the test will fail. Optionally you can add a `<testname>.desc` file which can contain a description of what the testcase tests.
