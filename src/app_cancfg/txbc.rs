#[doc = "Register `TXBC` reader"]
pub type R = crate::R<TxbcSpec>;
#[doc = "Register `TXBC` writer"]
pub type W = crate::W<TxbcSpec>;
#[doc = "Field `NU59` reader - 1:0\\]
Reserved"]
pub type Nu59R = crate::FieldReader;
#[doc = "Field `NU59` writer - 1:0\\]
Reserved"]
pub type Nu59W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TBSA` reader - 15:2\\]
Tx Buffers Start Address"]
pub type TbsaR = crate::FieldReader<u16>;
#[doc = "Field `TBSA` writer - 15:2\\]
Tx Buffers Start Address"]
pub type TbsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `NDTB` reader - 21:16\\]
Number of Dedicated Transmit Buffers"]
pub type NdtbR = crate::FieldReader;
#[doc = "Field `NDTB` writer - 21:16\\]
Number of Dedicated Transmit Buffers"]
pub type NdtbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU60` reader - 23:22\\]
Reserved"]
pub type Nu60R = crate::FieldReader;
#[doc = "Field `NU60` writer - 23:22\\]
Reserved"]
pub type Nu60W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TFQS` reader - 29:24\\]
Transmit FIFO/Queue Size"]
pub type TfqsR = crate::FieldReader;
#[doc = "Field `TFQS` writer - 29:24\\]
Transmit FIFO/Queue Size"]
pub type TfqsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFQM` reader - 30:30\\]
Tx FIFO/Queue Mode"]
pub type TfqmR = crate::BitReader;
#[doc = "Field `TFQM` writer - 30:30\\]
Tx FIFO/Queue Mode"]
pub type TfqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU61` reader - 31:31\\]
Reserved"]
pub type Nu61R = crate::BitReader;
#[doc = "Field `NU61` writer - 31:31\\]
Reserved"]
pub type Nu61W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu59(&self) -> Nu59R {
        Nu59R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Tx Buffers Start Address"]
    #[inline(always)]
    pub fn tbsa(&self) -> TbsaR {
        TbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of Dedicated Transmit Buffers"]
    #[inline(always)]
    pub fn ndtb(&self) -> NdtbR {
        NdtbR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved"]
    #[inline(always)]
    pub fn nu60(&self) -> Nu60R {
        Nu60R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Transmit FIFO/Queue Size"]
    #[inline(always)]
    pub fn tfqs(&self) -> TfqsR {
        TfqsR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Tx FIFO/Queue Mode"]
    #[inline(always)]
    pub fn tfqm(&self) -> TfqmR {
        TfqmR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved"]
    #[inline(always)]
    pub fn nu61(&self) -> Nu61R {
        Nu61R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu59(&mut self) -> Nu59W<TxbcSpec> {
        Nu59W::new(self, 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Tx Buffers Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn tbsa(&mut self) -> TbsaW<TxbcSpec> {
        TbsaW::new(self, 2)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of Dedicated Transmit Buffers"]
    #[inline(always)]
    #[must_use]
    pub fn ndtb(&mut self) -> NdtbW<TxbcSpec> {
        NdtbW::new(self, 16)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu60(&mut self) -> Nu60W<TxbcSpec> {
        Nu60W::new(self, 22)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Transmit FIFO/Queue Size"]
    #[inline(always)]
    #[must_use]
    pub fn tfqs(&mut self) -> TfqsW<TxbcSpec> {
        TfqsW::new(self, 24)
    }
    #[doc = "Bit 30 - 30:30\\]
Tx FIFO/Queue Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TfqmW<TxbcSpec> {
        TfqmW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu61(&mut self) -> Nu61W<TxbcSpec> {
        Nu61W::new(self, 31)
    }
}
#[doc = "TXBC\n\nYou can [`read`](crate::Reg::read) this register and get [`txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcSpec;
impl crate::RegisterSpec for TxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbc::R`](R) reader structure"]
impl crate::Readable for TxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`txbc::W`](W) writer structure"]
impl crate::Writable for TxbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBC to value 0"]
impl crate::Resettable for TxbcSpec {
    const RESET_VALUE: u32 = 0;
}
