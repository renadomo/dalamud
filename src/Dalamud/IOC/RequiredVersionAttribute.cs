using System;

namespace Dalamud.IOC
{
    [AttributeUsage( AttributeTargets.Parameter )]
    public class RequiredVersionAttribute : Attribute
    {
        public readonly Version Version;

        public RequiredVersionAttribute( string version ) => ( Version ) = new( version );
    }
}
