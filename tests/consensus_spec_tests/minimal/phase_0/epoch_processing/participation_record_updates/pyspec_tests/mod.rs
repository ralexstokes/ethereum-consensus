// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::ParticipationRecordUpdatesHandler as TestRunner;
use crate::test_utils::TestCase;

#[test]
fn test_updated_participation_record() {
    let test_case = TestRunner::from("consensus-spec-tests/tests/minimal/phase0/epoch_processing/participation_record_updates/pyspec_tests/updated_participation_record");
    test_case.execute();
}
