#[doc = "Register `ABCNT` reader"]
pub type R = crate::R<AbcntSpec>;
#[doc = "Register `ABCNT` writer"]
pub type W = crate::W<AbcntSpec>;
#[doc = "Field `ACNT` reader - 15:0\\]
ACNT : number of bytes in 1st dimension: ACNT represents the number of bytes within the first dimension of a transfer. ACNT is a 16-bit unsigned value with valid values between 0 and 65535. Therefore the maximum number of bytes in an array is 65535 bytes (64K-1 bytes). ACNT must be greater than or equal to '1' for a TR to be submitted to TC. An ACNT of '0' is considered as either a null or dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. If the OPT.WIMODE bit is set then the ACNT field represents a word count. The CC must internally multiply by 4 to translate the word count to a byte count prior to submission to the TC. The 2 MSBs of the 16-bit ACNT are reserved and should always be written as 'b00 by the user. If user writes a value other than 0 it will still be treated as 0 since the multiply-by-4 operation (to translate between a word count and a byte count) will drop the 2 msbits. For dummy and null transfer definition the ACNT definition will disregard the 2 msbits. I.e. a programmed ACNT value of 0x8000 in WI-mode will be treated as 0 byte transfer resulting in null or dummy operation dependent on the state of BCNT and CCNT."]
pub type AcntR = crate::FieldReader<u16>;
#[doc = "Field `ACNT` writer - 15:0\\]
ACNT : number of bytes in 1st dimension: ACNT represents the number of bytes within the first dimension of a transfer. ACNT is a 16-bit unsigned value with valid values between 0 and 65535. Therefore the maximum number of bytes in an array is 65535 bytes (64K-1 bytes). ACNT must be greater than or equal to '1' for a TR to be submitted to TC. An ACNT of '0' is considered as either a null or dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. If the OPT.WIMODE bit is set then the ACNT field represents a word count. The CC must internally multiply by 4 to translate the word count to a byte count prior to submission to the TC. The 2 MSBs of the 16-bit ACNT are reserved and should always be written as 'b00 by the user. If user writes a value other than 0 it will still be treated as 0 since the multiply-by-4 operation (to translate between a word count and a byte count) will drop the 2 msbits. For dummy and null transfer definition the ACNT definition will disregard the 2 msbits. I.e. a programmed ACNT value of 0x8000 in WI-mode will be treated as 0 byte transfer resulting in null or dummy operation dependent on the state of BCNT and CCNT."]
pub type AcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BCNT` reader - 31:16\\]
BCNT : Count for 2nd Dimension: BCNT is a 16-bit unsigned value that specifies the number of arrays of length ACNT. For normal operation valid values for BCNT can be anywhere between 1 and 65535. Therefore the maximum number of arrays in a frame is 65535 (64K-1 arrays). BCNT=1 means 1 array in the frame and BCNT=0 means 0 arrays in the frame. In normal mode a BCNT of '0' is considered as either a Null or Dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. If the OPT.WIMODE bit is set then the programmed BCNT value will be incremented by '1' before submission to TC. I.e. 0 means 1 1 means 2 2 means 3 ..."]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `BCNT` writer - 31:16\\]
BCNT : Count for 2nd Dimension: BCNT is a 16-bit unsigned value that specifies the number of arrays of length ACNT. For normal operation valid values for BCNT can be anywhere between 1 and 65535. Therefore the maximum number of arrays in a frame is 65535 (64K-1 arrays). BCNT=1 means 1 array in the frame and BCNT=0 means 0 arrays in the frame. In normal mode a BCNT of '0' is considered as either a Null or Dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. If the OPT.WIMODE bit is set then the programmed BCNT value will be incremented by '1' before submission to TC. I.e. 0 means 1 1 means 2 2 means 3 ..."]
pub type BcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
ACNT : number of bytes in 1st dimension: ACNT represents the number of bytes within the first dimension of a transfer. ACNT is a 16-bit unsigned value with valid values between 0 and 65535. Therefore the maximum number of bytes in an array is 65535 bytes (64K-1 bytes). ACNT must be greater than or equal to '1' for a TR to be submitted to TC. An ACNT of '0' is considered as either a null or dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. If the OPT.WIMODE bit is set then the ACNT field represents a word count. The CC must internally multiply by 4 to translate the word count to a byte count prior to submission to the TC. The 2 MSBs of the 16-bit ACNT are reserved and should always be written as 'b00 by the user. If user writes a value other than 0 it will still be treated as 0 since the multiply-by-4 operation (to translate between a word count and a byte count) will drop the 2 msbits. For dummy and null transfer definition the ACNT definition will disregard the 2 msbits. I.e. a programmed ACNT value of 0x8000 in WI-mode will be treated as 0 byte transfer resulting in null or dummy operation dependent on the state of BCNT and CCNT."]
    #[inline(always)]
    pub fn acnt(&self) -> AcntR {
        AcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
BCNT : Count for 2nd Dimension: BCNT is a 16-bit unsigned value that specifies the number of arrays of length ACNT. For normal operation valid values for BCNT can be anywhere between 1 and 65535. Therefore the maximum number of arrays in a frame is 65535 (64K-1 arrays). BCNT=1 means 1 array in the frame and BCNT=0 means 0 arrays in the frame. In normal mode a BCNT of '0' is considered as either a Null or Dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. If the OPT.WIMODE bit is set then the programmed BCNT value will be incremented by '1' before submission to TC. I.e. 0 means 1 1 means 2 2 means 3 ..."]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
ACNT : number of bytes in 1st dimension: ACNT represents the number of bytes within the first dimension of a transfer. ACNT is a 16-bit unsigned value with valid values between 0 and 65535. Therefore the maximum number of bytes in an array is 65535 bytes (64K-1 bytes). ACNT must be greater than or equal to '1' for a TR to be submitted to TC. An ACNT of '0' is considered as either a null or dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. If the OPT.WIMODE bit is set then the ACNT field represents a word count. The CC must internally multiply by 4 to translate the word count to a byte count prior to submission to the TC. The 2 MSBs of the 16-bit ACNT are reserved and should always be written as 'b00 by the user. If user writes a value other than 0 it will still be treated as 0 since the multiply-by-4 operation (to translate between a word count and a byte count) will drop the 2 msbits. For dummy and null transfer definition the ACNT definition will disregard the 2 msbits. I.e. a programmed ACNT value of 0x8000 in WI-mode will be treated as 0 byte transfer resulting in null or dummy operation dependent on the state of BCNT and CCNT."]
    #[inline(always)]
    #[must_use]
    pub fn acnt(&mut self) -> AcntW<AbcntSpec> {
        AcntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
BCNT : Count for 2nd Dimension: BCNT is a 16-bit unsigned value that specifies the number of arrays of length ACNT. For normal operation valid values for BCNT can be anywhere between 1 and 65535. Therefore the maximum number of arrays in a frame is 65535 (64K-1 arrays). BCNT=1 means 1 array in the frame and BCNT=0 means 0 arrays in the frame. In normal mode a BCNT of '0' is considered as either a Null or Dummy transfer. A Dummy or Null transfer will generate a Completion code depending on the settings of the completion bit fields of the OPT field. If the OPT.WIMODE bit is set then the programmed BCNT value will be incremented by '1' before submission to TC. I.e. 0 means 1 1 means 2 2 means 3 ..."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt(&mut self) -> BcntW<AbcntSpec> {
        BcntW::new(self, 16)
    }
}
#[doc = "A and B byte count\n\nYou can [`read`](crate::Reg::read) this register and get [`abcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AbcntSpec;
impl crate::RegisterSpec for AbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abcnt::R`](R) reader structure"]
impl crate::Readable for AbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`abcnt::W`](W) writer structure"]
impl crate::Writable for AbcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABCNT to value 0"]
impl crate::Resettable for AbcntSpec {
    const RESET_VALUE: u32 = 0;
}
