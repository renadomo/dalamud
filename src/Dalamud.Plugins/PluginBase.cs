using System;
using System.Threading.Tasks;

namespace Dalamud.Plugins
{
    public abstract class PluginBase : IPlugin
    {
        public virtual ValueTask DisposeAsync() => ValueTask.CompletedTask;
    }
}
