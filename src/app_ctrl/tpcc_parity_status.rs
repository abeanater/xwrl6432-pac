#[doc = "Register `TPCC_PARITY_STATUS` reader"]
pub type R = crate::R<TpccParityStatusSpec>;
#[doc = "Register `TPCC_PARITY_STATUS` writer"]
pub type W = crate::W<TpccParityStatusSpec>;
#[doc = "Field `tpcc_a_parity_addr` reader - 7:0\\]
address where parity error happened for tpcca"]
pub type TpccAParityAddrR = crate::FieldReader;
#[doc = "Field `tpcc_a_parity_addr` writer - 7:0\\]
address where parity error happened for tpcca"]
pub type TpccAParityAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `tpcc_b_parity_addr` reader - 23:16\\]
address where parity error happened for tpccb"]
pub type TpccBParityAddrR = crate::FieldReader;
#[doc = "Field `tpcc_b_parity_addr` writer - 23:16\\]
address where parity error happened for tpccb"]
pub type TpccBParityAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
address where parity error happened for tpcca"]
    #[inline(always)]
    pub fn tpcc_a_parity_addr(&self) -> TpccAParityAddrR {
        TpccAParityAddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
address where parity error happened for tpccb"]
    #[inline(always)]
    pub fn tpcc_b_parity_addr(&self) -> TpccBParityAddrR {
        TpccBParityAddrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
address where parity error happened for tpcca"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_parity_addr(&mut self) -> TpccAParityAddrW<TpccParityStatusSpec> {
        TpccAParityAddrW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
address where parity error happened for tpccb"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_parity_addr(&mut self) -> TpccBParityAddrW<TpccParityStatusSpec> {
        TpccBParityAddrW::new(self, 16)
    }
}
#[doc = "TPCC_PARITY_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`tpcc_parity_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpcc_parity_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpccParityStatusSpec;
impl crate::RegisterSpec for TpccParityStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpcc_parity_status::R`](R) reader structure"]
impl crate::Readable for TpccParityStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`tpcc_parity_status::W`](W) writer structure"]
impl crate::Writable for TpccParityStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPCC_PARITY_STATUS to value 0"]
impl crate::Resettable for TpccParityStatusSpec {
    const RESET_VALUE: u32 = 0;
}
