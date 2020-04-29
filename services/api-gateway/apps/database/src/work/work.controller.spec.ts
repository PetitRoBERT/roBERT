import { Test, TestingModule } from '@nestjs/testing';
import { WorkController } from './work.controller';

describe('Work Controller', () => {
  let controller: WorkController;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [WorkController],
    }).compile();

    controller = module.get<WorkController>(WorkController);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });
});
