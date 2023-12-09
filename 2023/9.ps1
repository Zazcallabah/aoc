$lines = gc "$PsScriptRoot/9.txt"
function nextline
{
	param($l)
	$r = @()
	$first = $l[0]
	$l | select -skip 1 | %{
		$r += $_ - $first
		$first = $_
	}
	return $r
}

function allequal
{
	param($l)
	$first = $l[0]
	foreach($item in $l)
	{
		if($item -ne $first)
		{
			return $false
		}
	}
	return $true
}
function sequencer {
	param($startline)
	$lines = @()
	do {
		$lines += ,($startline)
		$startline = nextline $startline

	}while(!(allequal $startline))
	$lines+=,($startline)
	return $lines
}
function extender {
	param($lines)
	$lastline = $lines[$lines.length-1]
	$finalvalue = $lastline[0]
	if($lines.length -eq 1){
		return $finalvalue
	}

	($lines.length-2)..0 |%{
		$cline = $lines[$_]
		$lastval = $cline[$cline.length-1]
		$finalvalue += $lastval
	}
	$finalvalue
}

function prepender {
	param($lines)
	$lastline = $lines[$lines.length-1]
	$finalvalue = $lastline[0]
	if($lines.length -eq 1){
		return $finalvalue
	}

	($lines.length-2)..0 |%{
		$cline = $lines[$_]
		$lastval = $cline[0]
		$nextvalue =  $lastval - $finalvalue
		$finalvalue = $nextvalue
	}
	$finalvalue
}
function makeline {
	param($str)
	return ($str -split " ") |?{$_ -ne ""} |%{ [int]$_ }
}
describe "can allequal" {
	It "is true" {
		allequal @(2, 2, 2, 2, 2) | should be $true
	}
	It "is false" {
		allequal @(2, 2, 3, 2, 2) | should be $false
	}
}
describe "next line" {
	It "extracts next line" {
		nextline (makeline "10  13  16  21  30  45") | should be @(3, 3 , 5 , 9, 15)
	}
}
describe "sequence" {
	It "makes array" {
		$l = sequencer (makeline  "10  13  16  21  30  45")
		$l.length | should be 4
		$l[1] | should be @(3, 3, 5, 9, 15)
		$l[2] | should be @(0, 2, 4, 6)
		$l[3] | should be @(2, 2, 2)
	}
}
describe "extender" {
	it "finds last value" {
		$l = sequencer (makeline "0 3 6 9 12 15")
		extender $l | should be 18
	}
}
describe "prepender" {
	it "should prepend value" {
		$l = sequencer (makeline  "10  13  16  21  30  45")
		prepender $l | should be 5
	}
}
write-host "end calculation:"
$lines | %{
	$l = makeline $_
	$s = sequencer $l
	extender $s
} | measure -sum

write-host "start calculation:"
$lines | %{
	$l = makeline $_
	$s = sequencer $l
	prepender $s
} | measure -sum
# 19973 is too high