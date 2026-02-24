// TODO: aws_applicationcostprofiler_report_definition is not in hashicorp/aws
// (verified 2026-04-30 — `gh api 'search/code?q=aws_applicationcostprofiler_report_definition+repo:hashicorp/terraform-provider-aws'`
// returns total_count=0). The AWS Application Cost Profiler service was deprecated
// in 2024 and the terraform AWS provider never shipped a resource for it.
//
// The winterbaume-applicationcostprofiler crate is still useful at the SDK level
// (PutReportDefinition / DeleteReportDefinition / ListReportDefinitions are
// reachable via aws-sdk-rust), so the service remains registered in
// `harness.rs::test_services()` for sdk-driven tests. Only terraform E2E tests
// are absent.
