using System.Collections.Generic;
using AutoMapper;
using Commander.Data;
using Commander.Dtobjs;
using Commander.Models;
using Microsoft.AspNetCore.Mvc;

namespace Commander.Controllers
{
    // api/commands
    // [Route("api/[controller]")]
    [Route("api/commands")]
    [ApiController]
    public class CommandsController : ControllerBase
    {

        private readonly ICommanderRepo _repo;
        private readonly IMapper _mapper;

        public CommandsController(ICommanderRepo repository, IMapper mapper)
        {
            _repo = repository;
            _mapper = mapper;
        }

        // private readonly MockCommanderRepo _repository = new MockCommanderRepo();
        // GET api/commands
        [HttpGet]
        public ActionResult<IEnumerable<CommandReadDtobj>> GetAllCommands()
        {
            var commandItems = _repo.GetAllCommands();

            return Ok(_mapper.Map<IEnumerable<CommandReadDtobj>>(commandItems));
        }

        // GET api/commands/{id}
        [HttpGet("{id}")]
        public ActionResult<CommandReadDtobj> GetCommandById(int id)
        {
            var commandItem = _repo.GetCommandById(id);

            if(commandItem != null)
            {
                return Ok(_mapper.Map<CommandReadDtobj>(commandItem));
            }

            return NotFound();
        }

        // POST api/commands
        [HttpPost]
        public ActionResult <CommandReadDtobj> CreateCommand(CommandCreateDtobj commandCreateDtobj)
        {
            var commandModel = _mapper.Map<Command>(commandCreateDtobj);
            _repo.CreateCommand(commandModel);
            _repo.SaveChanges();

            var commandReadDto = _mapper.Map<CommandReadDtobj>(commandModel);
            return Ok(commandReadDto);
        }
    }
}
