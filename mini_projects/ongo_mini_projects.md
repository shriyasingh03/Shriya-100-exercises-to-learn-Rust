📅 Week 1 – Rust basics (Ch 1–3)
01_hello_world.rs – println! “Hello, world!”

02_vars_and_types.rs – Play with i32, f64, bool, char, String.

03_basic_functions.rs – add, multiply, is_even functions.

04_fizzbuzz.rs – Loop 1–100, print Fizz/Buzz/FizzBuzz.

05_max_in_list.rs – Function that returns max of a slice.

06_temperature_converter.rs – C ↔ F converter with a function.

07_area_of_circle.rs – Compute area of circle with radius input.

08_simple_calculator.rs – CLI: input +, -, *, / and two numbers.

09_grade_checker.rs – Take 0–100 number, print “A”, “B”, “C”, etc.

10_count_down.rs – Countdown from N to 0 using a loop.


📅 Week 2 – Ownership, borrowing, slices (Ch 4–5)
01_string_lengths.rs – Take a list of strings, return their lengths.

02_count_vowels.rs – Count vowels in a string, pass &str.

03_reverse_words.rs – Reverse each word in a sentence.

04_word_start_with.rs – Find all words starting with a given letter.

05_concatenate_sentences.rs – Join sentences with space, using Vec<String>.

06_replace_placeholder.rs – Replace placeholders in a template string.

07_filter_short_words.rs – Keep only words longer than N.

08_first_line.rs – Read multi‑line text, return first line.

09_word_counter_on_string.rs – Count words in a string (no file).

10_word_counter_cli.rs – CLI: read from stdin or args, count words.


📅 Week 3 – Structs, enums, Result/Option (Ch 6–8)
01_todo_struct.rs – Todo struct (id, title, done).

02_todo_status_enum.rs – Pending, Done, Cancelled.

03_todo_list_in_memory.rs – Vec<Todo>; add, list, mark done.

04_todo_cli_with_pattern_matching.rs – Use match on Option/Result.

05_todo_with_input.rs – Read from stdin to add/delete tasks.

06_simple_bank_account.rs – struct Account { balance: f64 } + deposit.

07_withdraw_result.rs – withdraw returns Result<f64, &str>.

08_parking_spot_enum.rs – Empty, Occupied, Reserved with match.

09_option_exercises.rs – 5–10 small Option/Result helper functions.

mini_projects/todo_cli/src/main.rs – Full CLI todo app with this week’s patterns.


📅 Week 4 – Error handling, generics, traits (Ch 9–10)
01_config_parser.rs – Parse key:value string into Config struct.

02_config_from_json.rs – Parse {"port": 8080, "host": "localhost"}.

03_config_validator.rs – Validate port in 1–65535, return Result<(), String>.

04_generic_max.rs – fn max<T>(a: T, b: T) -> T where T: Ord.

05_generic_swap.rs – fn swap<T>(a: &mut T, b: &mut T).

06_sum_generic_list.rs – fn sum<T>(v: &[T]) -> T where T: Add + Copy.

07_trait_printable.rs – Trait Printable with print(&self).

08_trait_serialize.rs – Trait Serialize with to_string(&self).

09_config_from_file.rs – Read a file, return Result<Config, Box<dyn Error>>.

mini_projects/config_parser/src/main.rs – Full CLI that reads a file and validates config.


📅 Week 5 – Testing, iterators, closures (Ch 11–13)
01_test_hello.rs – Add unit tests for 01_hello_world.rs.

02_test_word_count.rs – Test word_counter_cli.rs edge cases.

03_test_todo_cli.rs – Unit tests for todo_cli (add, list, done).

04_iterator_map.rs – Use map to square numbers in a Vec.

05_iterator_filter_map.rs – Filter even numbers, then map to strings.

06_iterators_with_strings.rs – Count words via .split_whitespace() + collect.

07_count_words_with_iterators.rs – Rewrite word_counter using iterators.

08_sum_squares_with_iterators.rs – Square even numbers and sum them.

09_closure_examples.rs – Several closure examples: |x| x * 2, etc.

10_iterator_challenge.rs – Take list of names, filter by length, sort, return top 3.


📅 Week 6 – Smart pointers, Box, Rc, RefCell, threading (Ch 12–15)
01_box_example.rs – Use Box<String> instead of plain String in a small example.

02_box_tree.rs – Simple binary tree using Box<Node>.

03_rc_example.rs – Use Rc<String> to share a string between structures.

04_rc_refcell_example.rs – Rc<RefCell<T>> to mutate shared data.

05_shared_counter.rs – Shared counter between multiple “owners”.

06_threaded_counter.rs – Increment a counter in parallel with thread::spawn.

07_threaded_file_counter.rs – Count words in multiple files in parallel.

08_arc_mutex_example.rs – Re‑write threaded_counter with Arc<Mutex<T>>.

09_smart_pointers_exercises.rs – 5–10 small exercises using Box, Rc, RefCell.

mini_projects/threaded_file_counter/src/main.rs – Full project counting words in multiple files concurrently.


📅 Week 7 – Revision + 100 exercises (mix of topics)
01_revision_exercises_1.rs – 5–10 ownership + borrowing revision problems.

02_revision_exercises_2.rs – Generic + trait revision.

03_revision_exercises_3.rs – Error‑handling revision (Result/Option).

04_revision_exercises_4.rs – Iterator + closure revision.

05_revision_exercises_5.rs – Smart pointer revision.

06_exercise_86.rs – One new algorithm / logic problem.

07_exercise_87.rs – Another algorithm problem.

08_exercise_88.rs – String‑based challenge.

09_exercise_89.rs – Vector‑based challenge.

10_exercise_90.rs – Struct + enum challenge.