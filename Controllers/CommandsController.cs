using System.Collections.Generic;
using AutoMapper;
using Commander.Data;
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
        public ActionResult<IEnumerable<Command>> GetAllCommands()
        {
            var commandItems = _repo.GetAllCommands();

            return Ok(commandItems);
        }

        // GET api/commands/{id}
        [HttpGet("{id}")]
        public ActionResult<Command> GetCommandById(int id)
        {
            var commandItem = _repo.GetCommandById(id);

            return Ok(commandItem);
        }
    }
}
