#[doc = "Register `BIDX` reader"]
pub type R = crate::R<BidxSpec>;
#[doc = "Register `BIDX` writer"]
pub type W = crate::W<BidxSpec>;
#[doc = "Field `SBIDX` reader - 15:0\\]
Source 2nd Dimension Index: SBIDX is a 16-bit signed value (2's complement) used for source address modification in between each array in the 2nd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the source array to the beginning of the next source array. It applies to both A-sync and AB-sync transfers."]
pub type SbidxR = crate::FieldReader<u16>;
#[doc = "Field `SBIDX` writer - 15:0\\]
Source 2nd Dimension Index: SBIDX is a 16-bit signed value (2's complement) used for source address modification in between each array in the 2nd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the source array to the beginning of the next source array. It applies to both A-sync and AB-sync transfers."]
pub type SbidxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DBIDX` reader - 31:16\\]
Destination 2nd Dimension Index: DBIDX is a 16-bit signed value (2's complement) used for destination address modification in between each array in the 2nd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the destination array to the beginning of the next destination array within the current frame. It applies to both A-Sync and AB-Sync transfers."]
pub type DbidxR = crate::FieldReader<u16>;
#[doc = "Field `DBIDX` writer - 31:16\\]
Destination 2nd Dimension Index: DBIDX is a 16-bit signed value (2's complement) used for destination address modification in between each array in the 2nd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the destination array to the beginning of the next destination array within the current frame. It applies to both A-Sync and AB-Sync transfers."]
pub type DbidxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Source 2nd Dimension Index: SBIDX is a 16-bit signed value (2's complement) used for source address modification in between each array in the 2nd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the source array to the beginning of the next source array. It applies to both A-sync and AB-sync transfers."]
    #[inline(always)]
    pub fn sbidx(&self) -> SbidxR {
        SbidxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Destination 2nd Dimension Index: DBIDX is a 16-bit signed value (2's complement) used for destination address modification in between each array in the 2nd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the destination array to the beginning of the next destination array within the current frame. It applies to both A-Sync and AB-Sync transfers."]
    #[inline(always)]
    pub fn dbidx(&self) -> DbidxR {
        DbidxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Source 2nd Dimension Index: SBIDX is a 16-bit signed value (2's complement) used for source address modification in between each array in the 2nd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the source array to the beginning of the next source array. It applies to both A-sync and AB-sync transfers."]
    #[inline(always)]
    #[must_use]
    pub fn sbidx(&mut self) -> SbidxW<BidxSpec> {
        SbidxW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Destination 2nd Dimension Index: DBIDX is a 16-bit signed value (2's complement) used for destination address modification in between each array in the 2nd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the destination array to the beginning of the next destination array within the current frame. It applies to both A-Sync and AB-Sync transfers."]
    #[inline(always)]
    #[must_use]
    pub fn dbidx(&mut self) -> DbidxW<BidxSpec> {
        DbidxW::new(self, 16)
    }
}
#[doc = "Register description is not available\n\nYou can [`read`](crate::Reg::read) this register and get [`bidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BidxSpec;
impl crate::RegisterSpec for BidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bidx::R`](R) reader structure"]
impl crate::Readable for BidxSpec {}
#[doc = "`write(|w| ..)` method takes [`bidx::W`](W) writer structure"]
impl crate::Writable for BidxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIDX to value 0"]
impl crate::Resettable for BidxSpec {
    const RESET_VALUE: u32 = 0;
}
