using System;

namespace Dalamud.Plugins
{
    /// <summary>
    /// 
    /// </summary>
    public enum PluginLoadTime
    {
        /// <summary>
        /// 
        /// </summary>
        Early,

        /// <summary>
        /// 
        /// </summary>
        Game,
    }

    [AttributeUsage(AttributeTargets.Assembly)]
    public class PluginLoadTimeAttribute : Attribute
    {
        public PluginLoadTime LoadAt { get; init; }

        public PluginLoadTimeAttribute(PluginLoadTime loadAt)
        {
            LoadAt = loadAt;
        }
    }
}
