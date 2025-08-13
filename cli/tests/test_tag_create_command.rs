// Copyright 2025 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::common::TestEnvironment;

#[test]
fn test_tag_create() {
    let test_env = TestEnvironment::default();
    test_env.run_jj_in(".", ["git", "init", "repo"]).success();
    let work_dir = test_env.work_dir("repo");
    // Add commit
    work_dir.run_jj(["new", "root()", "-mcommit1"]).success();

    // There should be no tags
    let mut list_output = work_dir.run_jj(["tag", "list"]);
    insta::assert_snapshot!(list_output, @r"");

    // Create tag
    let create_output = work_dir
        .run_jj(["tag", "create", "my-tag", "-r", "@"])
        .success();
    insta::assert_snapshot!(create_output, @r"
    ------- stderr -------
    Hint: Tag create
    [EOF]
    ");

    // There should be the created tag
    // list_output = work_dir.run_jj(["tag", "list"]);
    // insta::assert_snapshot!(list_output, @r"
    // my-tag: rlvkpnrz 893e67dc (empty) commit1
    //  ");

}
