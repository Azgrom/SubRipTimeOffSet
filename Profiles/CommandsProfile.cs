using AutoMapper;
using Commander.Dtobjs;
using Commander.Models;

namespace Commander.Profiles
{
    public class CommandsProfile : Profile
    {
        public CommandsProfile()
        {
            // Source -> Target
            CreateMap<Command, CommandReadDtobj>();
            CreateMap<CommandCreateDtobj, Command>();
            CreateMap<CommandUpdateDtobj, Command>();
        }
    }
}
