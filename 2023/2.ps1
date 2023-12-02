
function possible {
    param($d)
    return $d.maxr -le 12 -and $d.maxg -le 13 -and $d.maxb -le 14
}
$regexIdMatcher = [regex]"^Game (\d+):"
function ParseLine {
    param($s)
    $idmatch = $regexIdMatcher.Match($s)
    $groups = $s.substring($idmatch.length+1) -split ";" |%{$_.trim()}
    $list = @($groups | %{ $_ -split "," |%{$_.trim()}})
    $mr = $list |?{$_.endswith("red")} | %{[int]($_ -split " ")[0]} | measure -maximum | select -expandproperty maximum
    $mg = $list |?{$_.endswith("green")} | %{[int]($_ -split " ")[0]} | measure -maximum | select -expandproperty maximum
    $mb = $list |?{$_.endswith("blue")} | %{[int]($_ -split " ")[0]} | measure -maximum | select -expandproperty maximum



    return new-object psobject -property @{
        "id"=$idmatch.Groups[1].value;
        "list"=$list;
        "maxR"=$mr;
        "maxG"=$mg;
        "maxB"=$mb;
    }
}
function sumpossible {
    param($data)
    $data |%{ ParseLine $_ }|?{ Possible $_ }| %{$_.id}|measure -sum | select -expandproperty sum
}

function powerline {
    param($line)
    return $line.maxr *$line.maxg*$line.maxb
}

Describe "parser" {
    It "parses line correctly" {
        $line = ParseLine "Game 83: 14 red, 2 green; 3 blue, 16 red, 2 green; 4 green, 13 red, 1 blue"
        $line.id | Should be 83
        $line.maxr | should be 16
    }
}
Describe "summer" {
    $testdata = @"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"@
it "can parse testdata" {
    $lines=($testdata -split "`n") |%{ ParseLine $_ }
    $lines.length | should be 5
    $lines[3].id | should be 4
    $lines[3].maxg | should be 3
    $poss= $lines | ?{ Possible $_ }
    $poss | %{ $_.id } | measure -sum | select -expandproperty sum | should be 8
}

it "can calc power"{
    $lines=($testdata -split "`n") |%{ ParseLine $_ }
    powerline $lines[0] | should be 48

}

    it "counts correct games possible" {
        sumpossible($testdata -split "`n") | should be 8
    }
}

$data = gc -Encoding utf8 ./2.txt
write-host "sum ids possible $(sumpossible($data))"
 # 3461 is to high

$lines = $data |%{ParseLine $_}
$powers = $lines | %{powerline $_ }

write-host "power min set $($powers | measure -sum | select -expandproperty sum )"