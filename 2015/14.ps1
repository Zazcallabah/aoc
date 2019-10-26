$data = "Dancer can fly 27 km/s for 5 seconds, but then must rest for 132 seconds.
Cupid can fly 22 km/s for 2 seconds, but then must rest for 41 seconds.
Rudolph can fly 11 km/s for 5 seconds, but then must rest for 48 seconds.
Donner can fly 28 km/s for 5 seconds, but then must rest for 134 seconds.
Dasher can fly 4 km/s for 16 seconds, but then must rest for 55 seconds.
Blitzen can fly 14 km/s for 3 seconds, but then must rest for 38 seconds.
Prancer can fly 3 km/s for 21 seconds, but then must rest for 40 seconds.
Comet can fly 18 km/s for 6 seconds, but then must rest for 103 seconds.
Vixen can fly 18 km/s for 5 seconds, but then must rest for 84 seconds."

function Calculatus
{
	param($name,$speed,$stamina,$rest,$finish)

	# concepts:
	# wind - as in "second wind" https://www.youtube.com/watch?v=rN3CY6TsYp8#t=25
	# sprint - the final segment before the race ends
	$numWinds = [Math]::floor($finish/($stamina+$rest))
	$sprintStart = $numWinds * ($stamina+$rest)
	$fullSprint = $finish-$sprintstart -ge $stamina

	if( $fullSprint )
	{
		$numWinds++
		$extra = 0
	}
	else
	{
		$extra = ($finish-$sprintstart) * $speed
	}

	$distance = $numWinds * ($speed*$stamina) + $extra
	$distance
}

$r = [regex]"(\w+)\D+(\d+)\D+(\d+)\D+(\d+)"
$after = 2503
$data -split "`n" | %{
	$match = $r.Match( $_ )
	$name = $match.Groups[1].Value
	$speed = [int]$match.Groups[2].Value
	$stamina = [int]$match.Groups[3].Value
	$rest = [int]$match.Groups[4].Value

	Calculatus $name $speed $stamina $rest $after
} | sort-object

function Get-Data
{
	param($data)
	$r = [regex]"(\w+)\D+(\d+)\D+(\d+)\D+(\d+)"
	$data -split "`n" | %{
		$match = $r.Match( $_ )
		new-object psobject -property @{
			"name" = $match.Groups[1].Value;
			"speed" = [int]$match.Groups[2].Value;
			"stamina" = [int]$match.Groups[3].Value;
			"rest" = [int]$match.Groups[4].Value;
			"interval" = [int]$match.Groups[3].Value + [int]$match.Groups[4].Value;
			"score" = 0;
			"distance" = 0;
		}
	}
}

function MoveDeer
{
	param($deer,$second)

	$resting = ($second-1) % $deer.interval -ge $deer.stamina

	if(!$resting)
	{
		$deer.distance += $deer.speed
	}
}

function TestDeer
{
	Get-Data "Comet can fly 2 km/s for 1 seconds, but then must rest for 9 seconds."
}
Describe "MoveDeer" {
	It "can move 1st second" {
		$d = TestDeer
		MoveDeer $d 1
		$d.distance | should be 2
	}
	It "wont move 2nd second" {
		$d = TestDeer
		MoveDeer $d 2
		$d.distance | should be 0
	}
	It "wont move 10th second" {
		$d = TestDeer
		MoveDeer $d 10
		$d.distance | should be 0
	}
	It "moves again 11th second" {
		$d = TestDeer
		MoveDeer $d 11
		$d.distance | should be 2
	}
}

function StepSecond
{
	param( $deers, $sec )
	$maxdistance = 0
	$deers | %{
		MoveDeer $_ $sec
		if( $_.distance -gt $maxdistance )
		{
			$maxdistance = $_.distance
		}
	}
	$deers | ?{ $_.distance -eq $maxdistance } | %{ $_.score++ }
}

Describe "StepSecond" {
	$deers = Get-Data "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."

	It "ticks one second" {
		StepSecond $deers 1
		$deers[0].distance | should be 14
		$deers[1].distance | should be 16
		$deers[0].score | should be 0
		$deers[1].score | should be 1
	}
	It "ticks nine more seconds" {
		2..10 | %{ StepSecond $deers $_ }
		$deers[0].distance | should be 140
		$deers[1].distance | should be 160
	}
	It "ticks to eleven" {
		StepSecond $deers 11
		$deers[0].distance | should be 140
		$deers[1].distance | should be 176
	}

	$deers = Get-Data "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 14 km/s for 11 seconds, but then must rest for 162 seconds."

	It "handles ties" {
		StepSecond $deers 1
		$deers[0].distance | should be 14
		$deers[1].distance | should be 14
		$deers[0].score | should be 1
		$deers[1].score | should be 1
	}

	$deers = Get-Data "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."

	It "ticks to 140" {
		1..140 | %{ StepSecond $deers $_ }
		$deers[0].score | should be 1
		$deers[1].score | should be 139
	}
}



$deers = Get-Data $data

1..2503 | %{
	StepSecond $deers $_
}

$deers | sort score | format-table
