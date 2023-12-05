$cardmatcher = [regex]"Card +(\d+):"
function cardValue
{
	param($card)
	$count = $card.wincount
	if($count -eq 0)
	{
		return 0;
	}

	return [math]::pow(2, $count-1)
}
function cardWinCount
{
	param($card)
	return $card.mine | ?{
		$card.winners.contains($_)
	} | measure | select -expandproperty count
}

function getCard
{
	param($str)
	$m = $cardmatcher.Match($str)
	$id= [int]$m.groups[1].value
	$split = $str.substring($m.length).split("|")
	$winners = $split[0] -split " " | ?{ $_-ne"" } | %{ [int]$_.trim() }
	$mine = $split[1] -split " " | ?{ $_ -ne "" } | %{ [int]$_.trim() }
	$card = new-object psobject -property @{id=$id; winners=$winners; mine=$mine }
	$value = cardwincount $card
	$card | add-member -membertype noteproperty -name "wincount" -Value $value
	$card | add-member -membertype noteproperty -name "Copies" -Value 1
	return $card
}
function getCards
{
	param($str)
	return $str -split "`n" | %{ getcard $_ }
}
describe "cardvalue"{
	it "get correct value" {
		$c = getCard "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
		$value = cardValue $c
		$value | should be 8
	}
}
describe "getcard" {
	it "parses input" {
		$r = getCard "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"
		$r.id | should be 4
	}
}
function crunchValues
{
	param($cards)
	for($i=0; $i -lt $cards.length; $i+=1)
	{
		$wincount = $cards[$i].wincount
		$copies = $cards[$i].copies

		for($c=1; $c -le $wincount; $c+=1)
		{
			$cards[$i+$c].copies += $copies
		}
	}
}
$cards = gc "$psscriptroot/4.txt" -encoding utf8 | %{ getCard $_ }
write-host "sum: $($cards |%{cardvalue $_}| measure -sum | select -ExpandProperty sum)"

describe "cards" {
	it "collects copies values" {
		$testcards = getcards "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
		$testcards[3].id | should be 4
		crunchvalues $testcards
		$copies = $testcards.copies | measure -sum
		$copies.sum | should be 30
	}
}

write-host "crunching..."
crunchvalues $cards
$sum = $cards.copies | measure -sum
write-host "count: $($sum.sum)"