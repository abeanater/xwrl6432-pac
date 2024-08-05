#[doc = "Register `CCNT` reader"]
pub type R = crate::R<CcntSpec>;
#[doc = "Register `CCNT` writer"]
pub type W = crate::W<CcntSpec>;
#[doc = "Field `CCNT` reader - 15:0\\]
CCNT : Count for 3rd Dimension: CCNT is a 16-bit unsigned value that specifies the number of frames in a block. Valid values for CCNT can be anywhere between 1 and 65535. Therefore the maximum number of frames in a block is 65535 (64K-1 frames). CCNT of '1' means '1' frame in the block and CCNT of '0' means '0' frames in the block. A CCNT value of '0' is considered as either a null or dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. WIMODE has no affect on CCNT operation."]
pub type CcntR = crate::FieldReader<u16>;
#[doc = "Field `CCNT` writer - 15:0\\]
CCNT : Count for 3rd Dimension: CCNT is a 16-bit unsigned value that specifies the number of frames in a block. Valid values for CCNT can be anywhere between 1 and 65535. Therefore the maximum number of frames in a block is 65535 (64K-1 frames). CCNT of '1' means '1' frame in the block and CCNT of '0' means '0' frames in the block. A CCNT value of '0' is considered as either a null or dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. WIMODE has no affect on CCNT operation."]
pub type CcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RES86` reader - 31:16\\]
RESERVE FIELD"]
pub type Res86R = crate::FieldReader<u16>;
#[doc = "Field `RES86` writer - 31:16\\]
RESERVE FIELD"]
pub type Res86W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
CCNT : Count for 3rd Dimension: CCNT is a 16-bit unsigned value that specifies the number of frames in a block. Valid values for CCNT can be anywhere between 1 and 65535. Therefore the maximum number of frames in a block is 65535 (64K-1 frames). CCNT of '1' means '1' frame in the block and CCNT of '0' means '0' frames in the block. A CCNT value of '0' is considered as either a null or dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. WIMODE has no affect on CCNT operation."]
    #[inline(always)]
    pub fn ccnt(&self) -> CcntR {
        CcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res86(&self) -> Res86R {
        Res86R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
CCNT : Count for 3rd Dimension: CCNT is a 16-bit unsigned value that specifies the number of frames in a block. Valid values for CCNT can be anywhere between 1 and 65535. Therefore the maximum number of frames in a block is 65535 (64K-1 frames). CCNT of '1' means '1' frame in the block and CCNT of '0' means '0' frames in the block. A CCNT value of '0' is considered as either a null or dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. WIMODE has no affect on CCNT operation."]
    #[inline(always)]
    #[must_use]
    pub fn ccnt(&mut self) -> CcntW<CcntSpec> {
        CcntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res86(&mut self) -> Res86W<CcntSpec> {
        Res86W::new(self, 16)
    }
}
#[doc = "C byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcntSpec;
impl crate::RegisterSpec for CcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt::R`](R) reader structure"]
impl crate::Readable for CcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ccnt::W`](W) writer structure"]
impl crate::Writable for CcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCNT to value 0"]
impl crate::Resettable for CcntSpec {
    const RESET_VALUE: u32 = 0;
}
