using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Dalamud.Plugins
{
    class PluginModules
    {
        Dictionary<Type, object> m_stuff = new Dictionary<Type, object>(); // TODO

        public void AddModule<T>() where T : new()
        {
            throw new NotImplementedException();
        }

        public void AddModule<T>(T module)
        {
            m_stuff[typeof(T)] = module;
            throw new NotImplementedException("TODO");
        }
    }
}
