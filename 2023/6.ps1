# Time:        44     80     65     72
# Distance:   208   1581   1050   1102

$times = @(44, 80, 65, 72)
$distances = @(208, 1581, 1050, 1102)

$racetime = 44806572
$racedistance = 208158110501102

# v = 0..44
# t = 44 - v
# d = v*t = (v)*(44-v) = 44v-v^2

# soo y=-x^2+44x-208


function getlimits
{
	param($t, $d)
	# y = -x^2 + $t*x - $d

	$p = $t / -2
	$q = -1 * $d

	$x1 = -1*$p + [Math]::sqrt($p*$p-$d)
	$x2 = -1*$p - [Math]::sqrt($p*$p-$d)

	# you can only ever start on whole seconds
	$lower = [math]::Floor($x2+1)
	# and end on whole seconds
	$upper = [math]::Ceiling($x1)

	return $upper - $lower
}

0..3 | %{
	$r=getlimits $times[$_] $distances[$_]
	write-host "race #$($_) can be won in $r ways"
}
$r = getlimits $racetime $racedistance
write-host "big race can be won in $($r) ways"

