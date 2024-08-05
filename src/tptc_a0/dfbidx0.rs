#[doc = "Register `DFBIDX0` reader"]
pub type R = crate::R<Dfbidx0Spec>;
#[doc = "Register `DFBIDX0` writer"]
pub type W = crate::W<Dfbidx0Spec>;
#[doc = "Field `SRC_BIDX_FOR` reader - 15:0\\]
Src B-Idx for Dest FIFO Register Set.#br#Value copied from PBIDX: B-Idx offset between Source arrays: Represents the offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT elements\\].#br#SBIDX is always used regardless of whether SAM is Increment or FIFO mode."]
pub type SrcBidxForR = crate::FieldReader<u16>;
#[doc = "Field `SRC_BIDX_FOR` writer - 15:0\\]
Src B-Idx for Dest FIFO Register Set.#br#Value copied from PBIDX: B-Idx offset between Source arrays: Represents the offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT elements\\].#br#SBIDX is always used regardless of whether SAM is Increment or FIFO mode."]
pub type SrcBidxForW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DEST_BIDX_FOR` reader - 31:16\\]
Dest B-Idx for Dest FIFO Register Set.#br#Value copied from PBIDX: B-Idx offset between Destination arrays: Represents the offset in bytes between the starting address of each destination array \\[recall that there are BCNT arrays of ACNT elements\\].#br#DBIDX is always used regardless of whether DAM is Increment or FIFO mode."]
pub type DestBidxForR = crate::FieldReader<u16>;
#[doc = "Field `DEST_BIDX_FOR` writer - 31:16\\]
Dest B-Idx for Dest FIFO Register Set.#br#Value copied from PBIDX: B-Idx offset between Destination arrays: Represents the offset in bytes between the starting address of each destination array \\[recall that there are BCNT arrays of ACNT elements\\].#br#DBIDX is always used regardless of whether DAM is Increment or FIFO mode."]
pub type DestBidxForW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Src B-Idx for Dest FIFO Register Set.#br#Value copied from PBIDX: B-Idx offset between Source arrays: Represents the offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT elements\\].#br#SBIDX is always used regardless of whether SAM is Increment or FIFO mode."]
    #[inline(always)]
    pub fn src_bidx_for(&self) -> SrcBidxForR {
        SrcBidxForR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Dest B-Idx for Dest FIFO Register Set.#br#Value copied from PBIDX: B-Idx offset between Destination arrays: Represents the offset in bytes between the starting address of each destination array \\[recall that there are BCNT arrays of ACNT elements\\].#br#DBIDX is always used regardless of whether DAM is Increment or FIFO mode."]
    #[inline(always)]
    pub fn dest_bidx_for(&self) -> DestBidxForR {
        DestBidxForR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Src B-Idx for Dest FIFO Register Set.#br#Value copied from PBIDX: B-Idx offset between Source arrays: Represents the offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT elements\\].#br#SBIDX is always used regardless of whether SAM is Increment or FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn src_bidx_for(&mut self) -> SrcBidxForW<Dfbidx0Spec> {
        SrcBidxForW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Dest B-Idx for Dest FIFO Register Set.#br#Value copied from PBIDX: B-Idx offset between Destination arrays: Represents the offset in bytes between the starting address of each destination array \\[recall that there are BCNT arrays of ACNT elements\\].#br#DBIDX is always used regardless of whether DAM is Increment or FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn dest_bidx_for(&mut self) -> DestBidxForW<Dfbidx0Spec> {
        DestBidxForW::new(self, 16)
    }
}
#[doc = "Dst FIFO Set B-Dim Idx\n\nYou can [`read`](crate::Reg::read) this register and get [`dfbidx0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfbidx0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dfbidx0Spec;
impl crate::RegisterSpec for Dfbidx0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfbidx0::R`](R) reader structure"]
impl crate::Readable for Dfbidx0Spec {}
#[doc = "`write(|w| ..)` method takes [`dfbidx0::W`](W) writer structure"]
impl crate::Writable for Dfbidx0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFBIDX0 to value 0"]
impl crate::Resettable for Dfbidx0Spec {
    const RESET_VALUE: u32 = 0;
}
