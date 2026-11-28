using System;
using System.IO;
using System.Text;

namespace Hello
{
    // A Lambda handler without dependencies: Stream in, Stream out needs no
    // serializer.
    public class Function
    {
        public Stream Handle(Stream input)
        {
            string evt = new StreamReader(input).ReadToEnd();
            string json = "{\"runtime\":\"dotnet\",\"version\":\"" + Environment.Version.Major + "\",\"event\":" + evt + "}";
            return new MemoryStream(Encoding.UTF8.GetBytes(json));
        }
    }
}
