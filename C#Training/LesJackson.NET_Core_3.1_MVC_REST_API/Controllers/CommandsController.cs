using System.Collections.Generic;
using AutoMapper;
using Commander.Data;
using Commander.Dtobjs;
using Commander.Models;
using Microsoft.AspNetCore.JsonPatch;
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
        [HttpGet("{id}", Name="GetCommandById")]
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

            return CreatedAtRoute(nameof(GetCommandById), new {Id = commandReadDto.Id}, commandReadDto);
        }

        // PUT api/commands/{id}
        [HttpPut("{id}")]
        public ActionResult UpdateCommand(int id, CommandUpdateDtobj commandUpdateDto)
        {
            var commandModelFromRepo = _repo.GetCommandById(id);
            if(commandModelFromRepo == null)
            {
                return NotFound();
            }

            _mapper.Map(commandUpdateDto, commandModelFromRepo);
            _repo.UpdateCommand(commandModelFromRepo);
            _repo.SaveChanges();

            return NoContent();
        }

        // PATCH api/commands/{id}
        [HttpPatch("{id}")]
        public ActionResult PartialCommandUpdate(int id, JsonPatchDocument<CommandUpdateDtobj> patchDoc) 
        { 
            var commandModelFromRepo = _repo.GetCommandById(id);
            if(commandModelFromRepo == null)
            {
                return NotFound();
            }

            var commandToPatch = _mapper.Map<CommandUpdateDtobj>(commandModelFromRepo);
            patchDoc.ApplyTo(commandToPatch, ModelState);

            if(!TryValidateModel(commandToPatch))
            {
                return ValidationProblem(ModelState);
            }

            _mapper.Map(commandToPatch, commandModelFromRepo);
            _repo.UpdateCommand(commandModelFromRepo);
            _repo.SaveChanges();

            return NoContent();
        }

        // DELETE api/commands/{id}
        [HttpDelete("{id}")]
        public ActionResult DeleteCommand(int id)
        {
            var commandModelFromRepo = _repo.GetCommandById(id);
            if(commandModelFromRepo == null)
            {
                return NotFound();
            }

            _repo.DeleteCommand(commandModelFromRepo);
            _repo.SaveChanges();

            return NoContent();
        }
    }
}
