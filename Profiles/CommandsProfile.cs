using AutoMapper;
using Commander.Dtobjs;
using Commander.Models;

namespace Commander.Profiles
{
    public class CommandsProfile : Profile
    {
        public CommandsProfile()
        {
            CreateMap<Command, CommandReadDtobj>();
        }
    }
}
