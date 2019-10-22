using System;
using System.Text;

namespace looksay
{
	class Program
	{
		static string Crunch(string str)
		{
			int c = 1;
			char last = str[0];
			StringBuilder output = new StringBuilder();
			for (var i = 1; i < str.Length; i++)
			{
				var p = str[i];
				if (p == last)
				{
					c++;
				}
				else
				{
					output.Append(c);
					output.Append(last);
					c = 1;
					last = p;
				}
			}
			output.Append(c);
			output.Append(last);
			return output.ToString();
		}

		static void Main(string[] args)
		{
			string data = "1113222113";
			for (var i = 1; i <= 50; i++)
			{
				data = Crunch(data);
				Console.WriteLine(i + ": " + data.Length);
			}
		}
	}
}
