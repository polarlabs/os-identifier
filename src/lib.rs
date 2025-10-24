mod windows;

// Algorithmus:
//
// Eingabestring nach Versionsnummer durchsuchen
// Versionsnummer ist [0-9]+[\.]

// todo: match OS with endoflife.date record

// Matches
// 10
// 10.0
// 10.0.26100

// 11.01
//^(?<major>0|[1-9]\d*)\.(?<minor>0|[1-9]\d*)\.(?<build>0|[1-9]\d*)
// Most advanced (approach to cover all):
// (?<=[\s_\-])(?<major>0|[1-9]\d*)\.(?<minor>[0-9]\d*)(?:\.(?<build>[0-9]\d*))?(?:\.(?<revision>[0-9]\d*))?(?=[\s_\-])
// (?<=[\s_\-])(?<major>0|[1-9][0-9A-Z]*)(?:\.(?<minor>[0-9]\d*))?(?:\.(?<build>[0-9]\d*))?(?:\.(?<revision>[0-9]\d*))?(?=[\s_\-])


// Windows 11 Pro 24H2

// Output
// Vendor: must have
// Product: must have
// Marketing Version: Optional
// Edition: Optional
// Date Version: Optional
// Version: Optional
// endoflife.date link: Optional
// Example: https://endoflife.date/api/v1/products/windows/releases/11-25h2-e
