using System;
using System.Text.RegularExpressions;
namespace lights
{
	class Program
	{

		static void Main(string[] args)
		{

			var data = System.IO.File.ReadAllLines("..\\6.txt");
			var lights = new bool[1000,1000];
			var conv = new Regex("(turn on|turn off|toggle) ([^,]+),([^ ]+) through ([^,]+),([^ ]+)");
			foreach( var line in data )
			{
				var m = conv.Match(line);
				var c = m.Groups[1].Value;
				var fx= int.Parse(m.Groups[2].Value);
				var fy= int.Parse(m.Groups[3].Value);
				var tx= int.Parse(m.Groups[4].Value);
				var ty= int.Parse(m.Groups[5].Value);

				if( fx > tx )
					throw new Exception("x");
				if( fy > ty )
					throw new Exception("y");
				if( c == "toggle" )
				{

					for( var x = fx; x<=tx; x++ )
					{
						for( var y = fy; y<=ty; y++ )
							lights[y,x]=!lights[y,x];
					}
				}
				else if( c == "turn off" )
				{
					for( var x = fx; x<=tx; x++ )
					{
						for( var y = fy; y<=ty; y++ )
							lights[y,x] = false;
					}
				}
				else if( c == "turn on" )
				{
					for( var x = fx; x<=tx; x++ )
					{
						for( var y = fy; y<=ty; y++ )
							lights[y,x] = true;
					}
				}
				else
				{
					throw new Exception("shit");
				}
			}
			int count = 0;
			foreach( var b in lights )
			{
				if( b )
					count++;
			}
			Console.WriteLine( "Done 1 "+count);


			var lights2 = new int[1000,1000];
			foreach( var line in data )
			{
				var m = conv.Match(line);
				var c = m.Groups[1].Value;
				var fx= int.Parse(m.Groups[2].Value);
				var fy= int.Parse(m.Groups[3].Value);
				var tx= int.Parse(m.Groups[4].Value);
				var ty= int.Parse(m.Groups[5].Value);

				if( fx > tx )
					throw new Exception("x");
				if( fy > ty )
					throw new Exception("y");
				if( c == "toggle" )
				{

					for( var x = fx; x<=tx; x++ )
					{
						for( var y = fy; y<=ty; y++ )
							lights2[y,x]+=2;
					}
				}
				else if( c == "turn off" )
				{
					for( var x = fx; x<=tx; x++ )
					{
						for( var y = fy; y<=ty; y++ )
						{
							if(lights2[y,x] > 0 )
							{
								lights2[y,x]--;
							}
						}
					}
				}
				else if( c == "turn on" )
				{
					for( var x = fx; x<=tx; x++ )
					{
						for( var y = fy; y<=ty; y++ )
							lights2[y,x]++;
					}
				}
				else
				{
					throw new Exception("shit");
				}
			}
			int count2 = 0;
			foreach( var b in lights2 )
			{
				count2+=b;
			}
			Console.WriteLine( "Done 2 "+count2);
		}
	}
}

