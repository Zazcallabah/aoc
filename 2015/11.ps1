#$data = "vzbxkghb"
$data = "vzbxxyzz"

# do it in base 26

function to26
{
	param($str)

	$sum = 0
	$place = 0
	for( $i=$str.length-1; $i -ge 0; $i-- )
	{
		$c = [int]$str[$i] - 97
		$sum += $c*[Math]::Pow(26,$place)
		$place++
	}
	return $sum
}


Describe "to26" {

	It "ba" {
		to26 "ba" | should be 26
	}

	It "a" {
		to26 "a" | should be 0
	}
	It "i" {
		to26 "i" | should be 8
	}
	It "l" {
		to26 "l" | should be 11
	}
	It "o" {
		to26 "o" | should be 14
	}
	It "b" {
		to26 "b" | should be 1
	}
	It "z" {
		to26 "z" | should be 25
	}
}

function from26
{
	param($sum)

	$str = ""

	while($sum -ge 1 )
	{
		[int]$rest = $sum % 26
		$str = ([char]($rest+97))+$str
		$sum = [Math]::Floor($sum/26)
	}

	return $str
}

Describe "from25" {
	It "switcheroo" {
		$num = to26 $data
		from26 $num | should be $data
	}
}

$pow = 0..10 | %{ [Math]::Pow(26,$_) }
function GetVal
{
	param($num,$ix)

	# dont call this function, paste the following instead
	[Math]::Floor($num / $pow[$ix] ) % 26
}

Describe "getval" {
	It "can find value ix 0" {
		$num = to26 "xxxxxxxa"
		GetVal $num 0 | should be 0
	}
	It "can find value ix 7" {
		$num = to26 "bxxxxxxx"
		GetVal $num 7 | should be 1
	}
	It "can find value ix middle" {
		$num = to26 "bbxbbbbb"
		GetVal $num 5 | should be 23
	}
}

function HasStraight
{
	#Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
	param($num)
	$first =[Math]::Floor($num / $pow[0] ) % 26
	$second =[Math]::Floor($num / $pow[1] ) % 26
	for( $ix=2;$ix -le 7; $ix++ )
	{
		$third = [Math]::Floor($num / $pow[$ix] ) % 26
		#remember we index backwards in $num
		$s = $second + 1
		$t = $third + 2
		if( $first -eq $s -and $s -eq $t )
		{
			return $true
		}
		$first = $second
		$second = $third
	}
	return $false
}

Describe "has straight" {
	It "handles straigt" {
		HasStraight (to26 "xxabcxxx") | should be $true
	}
	It "handles no straight" {
		HasStraight (to26 "zyxdabdx") | should be $false
	}
}
function HasTwoPairs
{
#Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
param($num)

	$hasOnePair = $false
	$first =[Math]::Floor($num / $pow[0] ) % 26
	for( $ix=1;$ix -le 7; $ix++ )
	{
		$val = [Math]::Floor($num / $pow[$ix] ) % 26
		if( $first -eq $val )
		{
			if( $hasOnePair )
			{
				return $true
			}
			else
			{
				$ix++
				$hasOnePair = $true
				$val = [Math]::Floor($num / $pow[$ix] ) % 26
			}
		}
		$first = $val
	}
	return $false
}

Describe "has two pairs" {
	It "handles no pair" {
		HasTwoPairs (to26 "abcdefgh") | should be $false
	}
	It "handles overlapping pairs" {
		HasTwoPairs (to26 "abcccfgh") | should be $false
	}
	It "handles non overlapping pairs" {
		HasTwoPairs (to26 "abccccgh") | should be $true
	}
	It "handles only one pair" {
		HasTwoPairs (to26 "abccxcgh") | should be $false
	}
}

function NoIllegalChars
{
#Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
	param($num)
	foreach( $ix in 0..7 )
	{
		$val = [Math]::Floor($num / $pow[$ix] ) % 26
		if( $val -eq 8 -or $val -eq 11 -or $val -eq 14 )
		{
			return $false
		}
	}
	return $true
}

Describe "no illegal chars" {
	It "finds invalid char" {
		$num = to26 "xxixxxxa"
		NoIllegalChars $num | should be $false
	}
	It "finds other char" {
		$num = to26 "xxoxxxa"
		NoIllegalChars $num | should be $false
	}
	It "finds third char" {
		$num = to26 "xxlxxa"
		NoIllegalChars $num | should be $false
	}
	It "finds valid" {
		$num = to26 "xxxxxxxa"
		NoIllegalChars $num | should be $true
	}
}

Describe "increment" {
	It "can add 1" {
		$num = to26 "xxxxyyyy"
		from26 ($num+1) | should be "xxxxyyyz"
	}
	It "handles 1 shift" {
		$num = to26 "xxxxyyyz"
		from26 ($num+1) | should be "xxxxyyza"
	}
	It "handles multiple shifts" {
		$num = to26 "xxxxzzzz"
		from26 ($num+1) | should be "xxxyaaaa"
	}
}


function IsValid
{
	param($num)

	return (NoIllegalChars $num) -and (HasStraight $num) -and (HasTwoPairs $num)
}

Describe "isvalid" {
	It "hijklmmn" {
		IsValid (to26 "hijklmmn") | should be $false
	}
	It "abbceffg" {
		IsValid (to26 "abbceffg") | should be $false
	}
	It "abbcegjk" {
		IsValid (to26 "abbcegjk") | should be $false
	}
}

function GetNext
{
	param($num)

	while(!(IsValid $num))
	{
		$num++
	}
	return $num
}

Describe "Getnext" {
	It "finds abcdffaa immediately" {
		$next = GetNext (to26 "xbcdffaa")
		from26 $next | should be "xbcdffaa"
	}
	It "finds abcdffaa after abcdefgh" {
		$next = GetNext (to26 "xbcdefgh")
		from26 $next | should be "xbcdffaa"
	}

}


#$next = GetNext (to26 $data)
$next = GetNext ((to26 $data) + 1)
$val = from26 $next

Write-Host $val